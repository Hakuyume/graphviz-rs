#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type time_t = __time_t;
pub type intptr_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gv_stack_t {
    pub base: *mut *mut libc::c_void,
    pub size: size_t,
    pub capacity: size_t,
}
pub const FIRST_ALLOCATION: C2RustUnnamed = 512;
pub type C2RustUnnamed = libc::c_uint;
pub type edgefn = Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vtx_data {
    pub nedges: libc::c_int,
    pub edges: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct treegen_s {
    pub N: libc::c_int,
    pub T: *mut libc::c_int,
    pub sp: gv_stack_t,
    pub tp: *mut tree_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_t {
    pub top: libc::c_int,
    pub root: libc::c_int,
    pub p: *mut libc::c_int,
}
pub type treegen_t = treegen_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pair {
    pub j: libc::c_int,
    pub d: libc::c_int,
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn stack_reset(mut stack: *mut gv_stack_t) {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void stack_reset(gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    free((*stack).base as *mut libc::c_void);
    memset(
        stack as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<gv_stack_t>() as libc::c_ulong,
    );
}
#[inline]
unsafe extern "C" fn stack_pop(mut stack: *mut gv_stack_t) -> *mut libc::c_void {
    let mut top: *mut libc::c_void = stack_top(stack);
    let ref mut fresh0 = (*stack).size;
    *fresh0 = (*fresh0).wrapping_sub(1);
    return top;
}
#[inline]
unsafe extern "C" fn stack_top(mut stack: *mut gv_stack_t) -> *mut libc::c_void {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void *stack_top(gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    if !stack_is_empty(stack)
        && !(b"access to top of an empty stack\0" as *const u8 as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"!stack_is_empty(stack) && \"access to top of an empty stack\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void *stack_top(gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    return *((*stack).base)
        .offset(
            ((*stack).size).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
}
#[inline]
unsafe extern "C" fn stack_push_or_exit(
    mut stack: *mut gv_stack_t,
    mut item: *mut libc::c_void,
) {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void stack_push_or_exit(gv_stack_t *, void *)\0"))
                .as_ptr(),
        );
    }
    let mut r: libc::c_int = stack_push(stack, item);
    if (r != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"stack_push failed: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(r),
        );
        graphviz_exit(1 as libc::c_int);
    }
}
#[inline]
unsafe extern "C" fn stack_push(
    mut stack: *mut gv_stack_t,
    mut item: *mut libc::c_void,
) -> libc::c_int {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"int stack_push(gv_stack_t *, void *)\0"))
                .as_ptr(),
        );
    }
    if (*stack).size == (*stack).capacity {
        if ((18446744073709551615 as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong) < (*stack).capacity)
            as libc::c_int as libc::c_long != 0
        {
            return 75 as libc::c_int;
        }
        let mut c: size_t = if (*stack).capacity == 0 as libc::c_int as libc::c_ulong {
            FIRST_ALLOCATION as libc::c_int as libc::c_ulong
        } else {
            (2 as libc::c_int as libc::c_ulong).wrapping_mul((*stack).capacity)
        };
        let mut b: *mut *mut libc::c_void = realloc(
            (*stack).base as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong).wrapping_mul(c),
        ) as *mut *mut libc::c_void;
        if (b == 0 as *mut libc::c_void as *mut *mut libc::c_void) as libc::c_int
            as libc::c_long != 0
        {
            return 12 as libc::c_int;
        }
        (*stack).capacity = c;
        let ref mut fresh1 = (*stack).base;
        *fresh1 = b;
    }
    if !((*stack).base).is_null() {} else {
        __assert_fail(
            b"stack->base != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"int stack_push(gv_stack_t *, void *)\0"))
                .as_ptr(),
        );
    }
    if (*stack).capacity > (*stack).size {} else {
        __assert_fail(
            b"stack->capacity > stack->size\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"int stack_push(gv_stack_t *, void *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh2 = *((*stack).base).offset((*stack).size as isize);
    *fresh2 = item;
    let ref mut fresh3 = (*stack).size;
    *fresh3 = (*fresh3).wrapping_add(1);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn stack_is_empty(mut stack: *const gv_stack_t) -> bool {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"_Bool stack_is_empty(const gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    return stack_size(stack) == 0 as libc::c_int as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn stack_size(mut stack: *const gv_stack_t) -> size_t {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            23 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"size_t stack_size(const gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    return (*stack).size;
}
#[no_mangle]
pub unsafe extern "C" fn makePath(mut n: libc::c_int, mut ef: edgefn) {
    let mut i: libc::c_int = 0;
    if n == 1 as libc::c_int {
        ef.expect("non-null function pointer")(1 as libc::c_int, 0 as libc::c_int);
        return;
    }
    i = 2 as libc::c_int;
    while i <= n {
        ef.expect("non-null function pointer")(i - 1 as libc::c_int, i);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn makeComplete(mut n: libc::c_int, mut ef: edgefn) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if n == 1 as libc::c_int {
        ef.expect("non-null function pointer")(1 as libc::c_int, 0 as libc::c_int);
        return;
    }
    i = 1 as libc::c_int;
    while i < n {
        j = i + 1 as libc::c_int;
        while j <= n {
            ef.expect("non-null function pointer")(i, j);
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn makeCircle(mut n: libc::c_int, mut ef: edgefn) {
    let mut i: libc::c_int = 0;
    if n < 3 as libc::c_int {
        fprintf(
            stderr,
            b"Warning: degenerate circle of %d vertices\n\0" as *const u8
                as *const libc::c_char,
            n,
        );
        makePath(n, ef);
        return;
    }
    i = 1 as libc::c_int;
    while i < n {
        ef.expect("non-null function pointer")(i, i + 1 as libc::c_int);
        i += 1;
    }
    ef.expect("non-null function pointer")(1 as libc::c_int, n);
}
#[no_mangle]
pub unsafe extern "C" fn makeStar(mut n: libc::c_int, mut ef: edgefn) {
    let mut i: libc::c_int = 0;
    if n < 3 as libc::c_int {
        fprintf(
            stderr,
            b"Warning: degenerate star of %d vertices\n\0" as *const u8
                as *const libc::c_char,
            n,
        );
        makePath(n, ef);
        return;
    }
    i = 2 as libc::c_int;
    while i <= n {
        ef.expect("non-null function pointer")(1 as libc::c_int, i);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn makeWheel(mut n: libc::c_int, mut ef: edgefn) {
    let mut i: libc::c_int = 0;
    if n < 4 as libc::c_int {
        fprintf(
            stderr,
            b"Warning: degenerate wheel of %d vertices\n\0" as *const u8
                as *const libc::c_char,
            n,
        );
        makeComplete(n, ef);
        return;
    }
    makeStar(n, ef);
    i = 2 as libc::c_int;
    while i < n {
        ef.expect("non-null function pointer")(i, i + 1 as libc::c_int);
        i += 1;
    }
    ef.expect("non-null function pointer")(2 as libc::c_int, n);
}
#[no_mangle]
pub unsafe extern "C" fn makeCompleteB(
    mut dim1: libc::c_int,
    mut dim2: libc::c_int,
    mut ef: edgefn,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= dim1 {
        j = 1 as libc::c_int;
        while j <= dim2 {
            ef.expect("non-null function pointer")(i, dim1 + j);
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn makeTorus(
    mut dim1: libc::c_int,
    mut dim2: libc::c_int,
    mut ef: edgefn,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= dim1 {
        j = 1 as libc::c_int;
        while j < dim2 {
            ef.expect("non-null function pointer")(n + j, n + j + 1 as libc::c_int);
            j += 1;
        }
        ef.expect("non-null function pointer")(n + 1 as libc::c_int, n + dim2);
        n += dim2;
        i += 1;
    }
    i = 1 as libc::c_int;
    while i <= dim2 {
        j = 1 as libc::c_int;
        while j < dim1 {
            ef
                .expect(
                    "non-null function pointer",
                )(dim2 * (j - 1 as libc::c_int) + i, dim2 * j + i);
            j += 1;
        }
        ef.expect("non-null function pointer")(i, dim2 * (dim1 - 1 as libc::c_int) + i);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn makeTwistedTorus(
    mut dim1: libc::c_int,
    mut dim2: libc::c_int,
    mut t1: libc::c_int,
    mut t2: libc::c_int,
    mut ef: edgefn,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut li: libc::c_int = 0;
    let mut lj: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < dim1 {
        j = 0 as libc::c_int;
        while j < dim2 {
            li = (i + t1) % dim1;
            lj = (j + 1 as libc::c_int) % dim2;
            ef
                .expect(
                    "non-null function pointer",
                )(i + j * dim1 + 1 as libc::c_int, li + lj * dim1 + 1 as libc::c_int);
            li = (i + 1 as libc::c_int) % dim1;
            lj = (j + t2) % dim2;
            ef
                .expect(
                    "non-null function pointer",
                )(i + j * dim1 + 1 as libc::c_int, li + lj * dim1 + 1 as libc::c_int);
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn makeCylinder(
    mut dim1: libc::c_int,
    mut dim2: libc::c_int,
    mut ef: edgefn,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= dim1 {
        j = 1 as libc::c_int;
        while j < dim2 {
            ef.expect("non-null function pointer")(n + j, n + j + 1 as libc::c_int);
            j += 1;
        }
        ef.expect("non-null function pointer")(n + 1 as libc::c_int, n + dim2);
        n += dim2;
        i += 1;
    }
    i = 1 as libc::c_int;
    while i <= dim2 {
        j = 1 as libc::c_int;
        while j < dim1 {
            ef
                .expect(
                    "non-null function pointer",
                )(dim2 * (j - 1 as libc::c_int) + i, dim2 * j + i);
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn makeSquareGrid(
    mut dim1: libc::c_int,
    mut dim2: libc::c_int,
    mut connect_corners: libc::c_int,
    mut partial: libc::c_int,
    mut ef: edgefn,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tl: libc::c_int = 0;
    let mut hd: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < dim1 {
        j = 0 as libc::c_int;
        while j < dim2 {
            tl = i * dim2 + j + 1 as libc::c_int;
            if j > 0 as libc::c_int
                && (partial == 0 || j <= 2 as libc::c_int * dim2 / 6 as libc::c_int
                    || j > 4 as libc::c_int * dim2 / 6 as libc::c_int
                    || i <= 2 as libc::c_int * dim1 / 6 as libc::c_int
                    || i > 4 as libc::c_int * dim1 / 6 as libc::c_int)
            {
                hd = i * dim2 + j;
                if tl < hd {
                    ef.expect("non-null function pointer")(tl, hd);
                }
            }
            if j < dim2 - 1 as libc::c_int
                && (partial == 0 || j < 2 as libc::c_int * dim2 / 6 as libc::c_int
                    || j >= 4 as libc::c_int * dim2 / 6 as libc::c_int
                    || i <= 2 as libc::c_int * dim1 / 6 as libc::c_int
                    || i > 4 as libc::c_int * dim1 / 6 as libc::c_int)
            {
                hd = i * dim2 + j + 2 as libc::c_int;
                if tl < hd {
                    ef.expect("non-null function pointer")(tl, hd);
                }
            }
            if i > 0 as libc::c_int {
                hd = (i - 1 as libc::c_int) * dim2 + j + 1 as libc::c_int;
                if tl < hd {
                    ef.expect("non-null function pointer")(tl, hd);
                }
            }
            if i < dim1 - 1 as libc::c_int {
                hd = (i + 1 as libc::c_int) * dim2 + j + 1 as libc::c_int;
                if tl < hd {
                    ef.expect("non-null function pointer")(tl, hd);
                }
            }
            if connect_corners == 1 as libc::c_int {
                if i == 0 as libc::c_int && j == 0 as libc::c_int {
                    hd = (dim1 - 1 as libc::c_int) * dim2 + dim2;
                    if tl < hd {
                        ef.expect("non-null function pointer")(tl, hd);
                    }
                } else if i == dim1 - 1 as libc::c_int && j == 0 as libc::c_int {
                    hd = dim2;
                    if tl < hd {
                        ef.expect("non-null function pointer")(tl, hd);
                    }
                } else if i == 0 as libc::c_int && j == dim2 - 1 as libc::c_int {
                    hd = (dim1 - 1 as libc::c_int) * dim2 + 1 as libc::c_int;
                    if tl < hd {
                        ef.expect("non-null function pointer")(tl, hd);
                    }
                } else if i == dim1 - 1 as libc::c_int && j == dim2 - 1 as libc::c_int {
                    hd = 1 as libc::c_int;
                    if tl < hd {
                        ef.expect("non-null function pointer")(tl, hd);
                    }
                }
            } else if connect_corners == 2 as libc::c_int {
                if i == 0 as libc::c_int && j == 0 as libc::c_int {
                    hd = dim2;
                    if tl < hd {
                        ef.expect("non-null function pointer")(tl, hd);
                    }
                } else if i == dim1 - 1 as libc::c_int && j == 0 as libc::c_int {
                    hd = (dim1 - 1 as libc::c_int) * dim2 + dim2;
                    if tl < hd {
                        ef.expect("non-null function pointer")(tl, hd);
                    }
                } else if i == 0 as libc::c_int && j == dim2 - 1 as libc::c_int {
                    hd = 1 as libc::c_int;
                    if tl < hd {
                        ef.expect("non-null function pointer")(tl, hd);
                    }
                } else if i == dim1 - 1 as libc::c_int && j == dim2 - 1 as libc::c_int {
                    hd = (dim1 - 1 as libc::c_int) * dim2 + 1 as libc::c_int;
                    if tl < hd {
                        ef.expect("non-null function pointer")(tl, hd);
                    }
                }
            }
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn ipow(mut base: libc::c_int, mut power: libc::c_int) -> libc::c_int {
    let mut ip: libc::c_int = 0;
    if power == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if power == 1 as libc::c_int {
        return base;
    }
    ip = base;
    power -= 1;
    loop {
        let fresh4 = power;
        power = power - 1;
        if !(fresh4 != 0) {
            break;
        }
        ip *= base;
    }
    return ip;
}
#[no_mangle]
pub unsafe extern "C" fn makeTree(
    mut depth: libc::c_int,
    mut nary: libc::c_int,
    mut ef: edgefn,
) {
    let mut n: libc::c_int = (ipow(nary, depth) - 1 as libc::c_int)
        / (nary - 1 as libc::c_int);
    let mut idx: libc::c_int = 2 as libc::c_int;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i <= n {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < nary {
            let fresh5 = idx;
            idx = idx + 1;
            ef.expect("non-null function pointer")(i, fresh5);
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn makeBinaryTree(mut depth: libc::c_int, mut ef: edgefn) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = ((1 as libc::c_int) << depth) - 1 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= n {
        ef.expect("non-null function pointer")(i, 2 as libc::c_int * i);
        ef
            .expect(
                "non-null function pointer",
            )(i, 2 as libc::c_int * i + 1 as libc::c_int);
        i += 1;
    }
}
unsafe extern "C" fn constructSierpinski(
    mut v1: libc::c_int,
    mut v2: libc::c_int,
    mut v3: libc::c_int,
    mut depth: libc::c_int,
    mut graph: *mut vtx_data,
) {
    static mut last_used_node_name: libc::c_int = 3 as libc::c_int;
    let mut v4: libc::c_int = 0;
    let mut v5: libc::c_int = 0;
    let mut v6: libc::c_int = 0;
    let mut nedges: libc::c_int = 0;
    if depth > 0 as libc::c_int {
        last_used_node_name += 1;
        v4 = last_used_node_name;
        last_used_node_name += 1;
        v5 = last_used_node_name;
        last_used_node_name += 1;
        v6 = last_used_node_name;
        constructSierpinski(v1, v4, v5, depth - 1 as libc::c_int, graph);
        constructSierpinski(v2, v5, v6, depth - 1 as libc::c_int, graph);
        constructSierpinski(v3, v4, v6, depth - 1 as libc::c_int, graph);
        return;
    }
    nedges = (*graph.offset(v1 as isize)).nedges;
    let fresh6 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v1 as isize)).edges).offset(fresh6 as isize) = v2;
    let fresh7 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v1 as isize)).edges).offset(fresh7 as isize) = v3;
    (*graph.offset(v1 as isize)).nedges = nedges;
    nedges = (*graph.offset(v2 as isize)).nedges;
    let fresh8 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v2 as isize)).edges).offset(fresh8 as isize) = v1;
    let fresh9 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v2 as isize)).edges).offset(fresh9 as isize) = v3;
    (*graph.offset(v2 as isize)).nedges = nedges;
    nedges = (*graph.offset(v3 as isize)).nedges;
    let fresh10 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v3 as isize)).edges).offset(fresh10 as isize) = v1;
    let fresh11 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v3 as isize)).edges).offset(fresh11 as isize) = v2;
    (*graph.offset(v3 as isize)).nedges = nedges;
}
#[no_mangle]
pub unsafe extern "C" fn makeSierpinski(mut depth: libc::c_int, mut ef: edgefn) {
    let mut graph: *mut vtx_data = 0 as *mut vtx_data;
    let mut edges: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    depth -= 1;
    n = 3 as libc::c_int
        * (1 as libc::c_int
            + ((pow(3.0f64, depth as libc::c_double) + 0.5f64) as libc::c_int
                - 1 as libc::c_int) / 2 as libc::c_int);
    graph = calloc(
        (n + 1 as libc::c_int) as libc::c_ulong,
        ::std::mem::size_of::<vtx_data>() as libc::c_ulong,
    ) as *mut vtx_data;
    edges = calloc(
        (4 as libc::c_int * n) as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 1 as libc::c_int;
    while i <= n {
        let ref mut fresh12 = (*graph.offset(i as isize)).edges;
        *fresh12 = edges;
        edges = edges.offset(4 as libc::c_int as isize);
        (*graph.offset(i as isize)).nedges = 0 as libc::c_int;
        i += 1;
    }
    constructSierpinski(
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        depth,
        graph,
    );
    i = 1 as libc::c_int;
    while i <= n {
        let mut nghbr: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < (*graph.offset(i as isize)).nedges {
            nghbr = *((*graph.offset(i as isize)).edges).offset(j as isize);
            if i < nghbr {
                ef.expect("non-null function pointer")(i, nghbr);
            }
            j += 1;
        }
        i += 1;
    }
    free((*graph.offset(1 as libc::c_int as isize)).edges as *mut libc::c_void);
    free(graph as *mut libc::c_void);
}
unsafe extern "C" fn constructTetrix(
    mut v1: libc::c_int,
    mut v2: libc::c_int,
    mut v3: libc::c_int,
    mut v4: libc::c_int,
    mut depth: libc::c_int,
    mut graph: *mut vtx_data,
) {
    static mut last_used_node_name: libc::c_int = 4 as libc::c_int;
    let mut v5: libc::c_int = 0;
    let mut v6: libc::c_int = 0;
    let mut v7: libc::c_int = 0;
    let mut v8: libc::c_int = 0;
    let mut v9: libc::c_int = 0;
    let mut v10: libc::c_int = 0;
    let mut nedges: libc::c_int = 0;
    if depth > 0 as libc::c_int {
        last_used_node_name += 1;
        v5 = last_used_node_name;
        last_used_node_name += 1;
        v6 = last_used_node_name;
        last_used_node_name += 1;
        v7 = last_used_node_name;
        last_used_node_name += 1;
        v8 = last_used_node_name;
        last_used_node_name += 1;
        v9 = last_used_node_name;
        last_used_node_name += 1;
        v10 = last_used_node_name;
        constructTetrix(v1, v5, v6, v8, depth - 1 as libc::c_int, graph);
        constructTetrix(v2, v6, v7, v9, depth - 1 as libc::c_int, graph);
        constructTetrix(v3, v5, v7, v10, depth - 1 as libc::c_int, graph);
        constructTetrix(v4, v8, v9, v10, depth - 1 as libc::c_int, graph);
        return;
    }
    nedges = (*graph.offset(v1 as isize)).nedges;
    let fresh13 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v1 as isize)).edges).offset(fresh13 as isize) = v2;
    let fresh14 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v1 as isize)).edges).offset(fresh14 as isize) = v3;
    let fresh15 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v1 as isize)).edges).offset(fresh15 as isize) = v4;
    (*graph.offset(v1 as isize)).nedges = nedges;
    nedges = (*graph.offset(v2 as isize)).nedges;
    let fresh16 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v2 as isize)).edges).offset(fresh16 as isize) = v1;
    let fresh17 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v2 as isize)).edges).offset(fresh17 as isize) = v3;
    let fresh18 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v2 as isize)).edges).offset(fresh18 as isize) = v4;
    (*graph.offset(v2 as isize)).nedges = nedges;
    nedges = (*graph.offset(v3 as isize)).nedges;
    let fresh19 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v3 as isize)).edges).offset(fresh19 as isize) = v1;
    let fresh20 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v3 as isize)).edges).offset(fresh20 as isize) = v2;
    let fresh21 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v3 as isize)).edges).offset(fresh21 as isize) = v4;
    (*graph.offset(v3 as isize)).nedges = nedges;
    nedges = (*graph.offset(v4 as isize)).nedges;
    let fresh22 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v4 as isize)).edges).offset(fresh22 as isize) = v1;
    let fresh23 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v4 as isize)).edges).offset(fresh23 as isize) = v2;
    let fresh24 = nedges;
    nedges = nedges + 1;
    *((*graph.offset(v4 as isize)).edges).offset(fresh24 as isize) = v3;
    (*graph.offset(v4 as isize)).nedges = nedges;
}
#[no_mangle]
pub unsafe extern "C" fn makeTetrix(mut depth: libc::c_int, mut ef: edgefn) {
    let mut graph: *mut vtx_data = 0 as *mut vtx_data;
    let mut edges: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    depth -= 1;
    n = 4 as libc::c_int
        + 2 as libc::c_int
            * ((pow(4.0f64, depth as libc::c_double) + 0.5f64) as libc::c_int
                - 1 as libc::c_int);
    graph = calloc(
        (n + 1 as libc::c_int) as libc::c_ulong,
        ::std::mem::size_of::<vtx_data>() as libc::c_ulong,
    ) as *mut vtx_data;
    edges = calloc(
        (6 as libc::c_int * n) as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 1 as libc::c_int;
    while i <= n {
        let ref mut fresh25 = (*graph.offset(i as isize)).edges;
        *fresh25 = edges;
        edges = edges.offset(6 as libc::c_int as isize);
        (*graph.offset(i as isize)).nedges = 0 as libc::c_int;
        i += 1;
    }
    constructTetrix(
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        depth,
        graph,
    );
    i = 1 as libc::c_int;
    while i <= n {
        let mut nghbr: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < (*graph.offset(i as isize)).nedges {
            nghbr = *((*graph.offset(i as isize)).edges).offset(j as isize);
            if i < nghbr {
                ef.expect("non-null function pointer")(i, nghbr);
            }
            j += 1;
        }
        i += 1;
    }
    free((*graph.offset(1 as libc::c_int as isize)).edges as *mut libc::c_void);
    free(graph as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn makeHypercube(mut dim: libc::c_int, mut ef: edgefn) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut neighbor: libc::c_int = 0;
    n = (1 as libc::c_int) << dim;
    i = 0 as libc::c_int;
    while i < n {
        j = 0 as libc::c_int;
        while j < dim {
            neighbor = (i ^ (1 as libc::c_int) << j) + 1 as libc::c_int;
            if i < neighbor {
                ef.expect("non-null function pointer")(i + 1 as libc::c_int, neighbor);
            }
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn makeTriMesh(mut sz: libc::c_int, mut ef: edgefn) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    if sz == 1 as libc::c_int {
        ef.expect("non-null function pointer")(1 as libc::c_int, 0 as libc::c_int);
        return;
    }
    ef.expect("non-null function pointer")(1 as libc::c_int, 2 as libc::c_int);
    ef.expect("non-null function pointer")(1 as libc::c_int, 3 as libc::c_int);
    idx = 2 as libc::c_int;
    i = 2 as libc::c_int;
    while i < sz {
        j = 1 as libc::c_int;
        while j <= i {
            ef.expect("non-null function pointer")(idx, idx + i);
            ef.expect("non-null function pointer")(idx, idx + i + 1 as libc::c_int);
            if j < i {
                ef.expect("non-null function pointer")(idx, idx + 1 as libc::c_int);
            }
            idx += 1;
            j += 1;
        }
        i += 1;
    }
    j = 1 as libc::c_int;
    while j < sz {
        ef.expect("non-null function pointer")(idx, idx + 1 as libc::c_int);
        idx += 1;
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn makeBall(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut ef: edgefn,
) {
    let mut i: libc::c_int = 0;
    let mut cap: libc::c_int = 0;
    makeCylinder(w, h, ef);
    i = 1 as libc::c_int;
    while i <= h {
        ef.expect("non-null function pointer")(0 as libc::c_int, i);
        i += 1;
    }
    cap = w * h + 1 as libc::c_int;
    i = (w - 1 as libc::c_int) * h + 1 as libc::c_int;
    while i <= w * h {
        ef.expect("non-null function pointer")(i, cap);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn makeRandom(
    mut h: libc::c_int,
    mut w: libc::c_int,
    mut ef: edgefn,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    srand(time(0 as *mut time_t) as libc::c_uint);
    type_0 = rand() % 2 as libc::c_int;
    depth = 0 as libc::c_int;
    size = depth;
    while size <= h {
        size += (1 as libc::c_int) << depth;
        depth += 1;
    }
    depth -= 1;
    if size > h {
        size -= (1 as libc::c_int) << depth;
        depth -= 1;
    }
    if type_0 != 0 {
        makeBinaryTree(depth, ef);
    } else {
        makePath(size, ef);
    }
    i = 3 as libc::c_int;
    while i <= size {
        j = 1 as libc::c_int;
        while j < i - 1 as libc::c_int {
            let mut th: libc::c_int = rand() % (size * size);
            if th <= w * w
                && (i < 5 as libc::c_int
                    || i > h - 4 as libc::c_int && j > h - 4 as libc::c_int) || th <= w
            {
                ef.expect("non-null function pointer")(j, i);
            }
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn makeMobius(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut ef: edgefn,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if h == 1 as libc::c_int {
        fprintf(
            stderr,
            b"Warning: degenerate Moebius strip of %d vertices\n\0" as *const u8
                as *const libc::c_char,
            w,
        );
        makePath(w, ef);
        return;
    }
    if w == 1 as libc::c_int {
        fprintf(
            stderr,
            b"Warning: degenerate Moebius strip of %d vertices\n\0" as *const u8
                as *const libc::c_char,
            h,
        );
        makePath(h, ef);
        return;
    }
    i = 0 as libc::c_int;
    while i < w - 1 as libc::c_int {
        j = 1 as libc::c_int;
        while j < h {
            ef
                .expect(
                    "non-null function pointer",
                )(j + i * h, j + (i + 1 as libc::c_int) * h);
            ef
                .expect(
                    "non-null function pointer",
                )(j + i * h, j + 1 as libc::c_int + i * h);
            j += 1;
        }
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < h {
        ef
            .expect(
                "non-null function pointer",
            )(
            i + (w - 1 as libc::c_int) * h,
            i + 1 as libc::c_int + (w - 1 as libc::c_int) * h,
        );
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < w {
        ef.expect("non-null function pointer")(i * h, (i + 1 as libc::c_int) * h);
        ef.expect("non-null function pointer")(i * h, (w - i) * h + 1 as libc::c_int);
        i += 1;
    }
    ef.expect("non-null function pointer")(1 as libc::c_int, w * h);
}
unsafe extern "C" fn mkTree(mut sz: libc::c_int) -> *mut tree_t {
    let mut tp: *mut tree_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<tree_t>() as libc::c_ulong,
    ) as *mut tree_t;
    (*tp).root = 0 as libc::c_int;
    (*tp).top = 0 as libc::c_int;
    let ref mut fresh26 = (*tp).p;
    *fresh26 = calloc(
        sz as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    return tp;
}
unsafe extern "C" fn freeTree(mut tp: *mut tree_t) {
    free((*tp).p as *mut libc::c_void);
    free(tp as *mut libc::c_void);
}
unsafe extern "C" fn resetTree(mut tp: *mut tree_t) {
    (*tp).root = 0 as libc::c_int;
    (*tp).top = 0 as libc::c_int;
}
unsafe extern "C" fn treeRoot(mut tp: *mut tree_t) -> libc::c_int {
    return (*tp).root;
}
unsafe extern "C" fn prevRoot(mut tp: *mut tree_t) -> libc::c_int {
    return *((*tp).p).offset((*tp).root as isize);
}
unsafe extern "C" fn treeSize(mut tp: *mut tree_t) -> libc::c_int {
    return (*tp).top - (*tp).root + 1 as libc::c_int;
}
unsafe extern "C" fn treeTop(mut tp: *mut tree_t) -> libc::c_int {
    return (*tp).top;
}
unsafe extern "C" fn treePop(mut tp: *mut tree_t) {
    (*tp).root = prevRoot(tp);
}
unsafe extern "C" fn addTree(mut tp: *mut tree_t, mut sz: libc::c_int) {
    *((*tp).p).offset(((*tp).top + 1 as libc::c_int) as isize) = (*tp).root;
    (*tp).root = (*tp).top + 1 as libc::c_int;
    (*tp).top += sz;
    if sz > 1 as libc::c_int {
        *((*tp).p).offset((*tp).top as isize) = (*tp).top - 1 as libc::c_int;
    }
}
unsafe extern "C" fn treeDup(mut tp: *mut tree_t, mut J: libc::c_int) {
    let mut M: libc::c_int = treeSize(tp);
    let mut L: libc::c_int = treeRoot(tp);
    let mut LL: libc::c_int = prevRoot(tp);
    let mut i: libc::c_int = 0;
    let mut LS: libc::c_int = L + (J - 1 as libc::c_int) * M - 1 as libc::c_int;
    i = L;
    while i <= LS {
        if (i - L) % M == 0 as libc::c_int {
            *((*tp).p).offset((i + M) as isize) = LL;
        } else {
            *((*tp).p).offset((i + M) as isize) = *((*tp).p).offset(i as isize) + M;
        }
        i += 1;
    }
    (*tp).top = LS + M;
}
unsafe extern "C" fn push(
    mut sp: *mut gv_stack_t,
    mut j: libc::c_int,
    mut d: libc::c_int,
) {
    let mut j_ptr: *mut libc::c_void = j as intptr_t as *mut libc::c_void;
    let mut d_ptr: *mut libc::c_void = d as intptr_t as *mut libc::c_void;
    stack_push_or_exit(sp, j_ptr);
    stack_push_or_exit(sp, d_ptr);
}
unsafe extern "C" fn pop(mut sp: *mut gv_stack_t) -> pair {
    let mut d: libc::c_int = stack_pop(sp) as intptr_t as libc::c_int;
    let mut j: libc::c_int = stack_pop(sp) as intptr_t as libc::c_int;
    return {
        let mut init = pair { j: j, d: d };
        init
    };
}
unsafe extern "C" fn genCnt(mut NN: libc::c_int) -> *mut libc::c_int {
    let mut T: *mut libc::c_int = calloc(
        (NN + 1 as libc::c_int) as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut D: libc::c_int = 0;
    let mut I: libc::c_int = 0;
    let mut J: libc::c_int = 0;
    let mut TD: libc::c_int = 0;
    let mut SUM: libc::c_int = 0;
    let mut NLAST: libc::c_int = 1 as libc::c_int;
    *T.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
    while NN > NLAST {
        SUM = 0 as libc::c_int;
        D = 1 as libc::c_int;
        while D <= NLAST {
            I = NLAST + 1 as libc::c_int;
            TD = *T.offset(D as isize) * D;
            J = 1 as libc::c_int;
            while J <= NLAST {
                I = I - D;
                if I <= 0 as libc::c_int {
                    break;
                }
                SUM += *T.offset(I as isize) * TD;
                J += 1;
            }
            D += 1;
        }
        NLAST += 1;
        *T.offset(NLAST as isize) = SUM / (NLAST - 1 as libc::c_int);
    }
    return T;
}
unsafe extern "C" fn drand() -> libc::c_double {
    let mut d: libc::c_double = 0.;
    d = rand() as libc::c_double;
    d = d / 2147483647 as libc::c_int as libc::c_double;
    return d;
}
unsafe extern "C" fn genTree(
    mut NN: libc::c_int,
    mut T: *mut libc::c_int,
    mut stack: *mut gv_stack_t,
    mut TREE: *mut tree_t,
) {
    let mut v: libc::c_double = 0.;
    let mut p: pair = pair { j: 0, d: 0 };
    let mut Z: libc::c_int = 0;
    let mut D: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut J: libc::c_int = 0;
    let mut TD: libc::c_int = 0;
    let mut M: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    N = NN;
    loop {
        while N > 2 as libc::c_int {
            v = ((N - 1 as libc::c_int) * *T.offset(N as isize)) as libc::c_double;
            Z = (v * drand()) as libc::c_int;
            D = 0 as libc::c_int;
            more = 1 as libc::c_int;
            loop {
                D += 1;
                TD = D * *T.offset(D as isize);
                M = N;
                J = 0 as libc::c_int;
                loop {
                    J += 1;
                    M -= D;
                    if M < 1 as libc::c_int {
                        break;
                    }
                    Z -= *T.offset(M as isize) * TD;
                    if Z < 0 as libc::c_int {
                        more = 0 as libc::c_int;
                    }
                    if !(Z >= 0 as libc::c_int) {
                        break;
                    }
                }
                if !(more != 0) {
                    break;
                }
            }
            push(stack, J, D);
            N = M;
        }
        addTree(TREE, N);
        loop {
            p = pop(stack);
            N = p.d;
            if N != 0 as libc::c_int {
                push(stack, p.j, 0 as libc::c_int);
                break;
            } else {
                J = p.j;
                if J > 1 as libc::c_int {
                    treeDup(TREE, J);
                }
                if treeTop(TREE) == NN {
                    return;
                }
                treePop(TREE);
            }
        }
    };
}
unsafe extern "C" fn writeTree(mut tp: *mut tree_t, mut ef: edgefn) {
    let mut i: libc::c_int = 0;
    i = 2 as libc::c_int;
    while i <= (*tp).top {
        ef.expect("non-null function pointer")(*((*tp).p).offset(i as isize), i);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn makeTreeGen(mut N: libc::c_int) -> *mut treegen_t {
    let mut tg: *mut treegen_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<treegen_t>() as libc::c_ulong,
    ) as *mut treegen_t;
    (*tg).N = N;
    let ref mut fresh27 = (*tg).T;
    *fresh27 = genCnt(N);
    (*tg)
        .sp = {
        let mut init = gv_stack_t {
            base: 0 as *mut *mut libc::c_void,
            size: 0,
            capacity: 0,
        };
        init
    };
    let ref mut fresh28 = (*tg).tp;
    *fresh28 = mkTree(N + 1 as libc::c_int);
    srand(time(0 as *mut time_t) as libc::c_uint);
    return tg;
}
#[no_mangle]
pub unsafe extern "C" fn makeRandomTree(mut tg: *mut treegen_t, mut ef: edgefn) {
    stack_reset(&mut (*tg).sp);
    resetTree((*tg).tp);
    genTree((*tg).N, (*tg).T, &mut (*tg).sp, (*tg).tp);
    writeTree((*tg).tp, ef);
}
#[no_mangle]
pub unsafe extern "C" fn freeTreeGen(mut tg: *mut treegen_t) {
    free((*tg).T as *mut libc::c_void);
    stack_reset(&mut (*tg).sp);
    freeTree((*tg).tp);
    free(tg as *mut libc::c_void);
}
