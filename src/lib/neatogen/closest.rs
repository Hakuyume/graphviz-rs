#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn rand() -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn quicksort_place(
        _: *mut libc::c_double,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
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
pub struct gv_stack_t {
    pub base: *mut *mut libc::c_void,
    pub size: size_t,
    pub capacity: size_t,
}
pub const FIRST_ALLOCATION: C2RustUnnamed = 512;
pub type C2RustUnnamed = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vtx_data {
    pub nedges: libc::c_int,
    pub edges: *mut libc::c_int,
    pub ewgts: *mut libc::c_float,
    pub eweights: *mut libc::c_float,
    pub edists: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pair {
    pub left: libc::c_int,
    pub right: libc::c_int,
    pub dist: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PairHeap {
    pub data: *mut Pair,
    pub heapSize: libc::c_int,
    pub maxSize: libc::c_int,
}
#[inline]
unsafe extern "C" fn gv_alloc(mut size: size_t) -> *mut libc::c_void {
    return gv_calloc(1 as libc::c_int as size_t, size);
}
#[inline]
unsafe extern "C" fn gv_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = calloc(nmemb, size);
    if (nmemb > 0 as libc::c_int as libc::c_ulong
        && size > 0 as libc::c_int as libc::c_ulong && p.is_null()) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return p;
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
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
        let ref mut fresh0 = (*stack).base;
        *fresh0 = b;
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
    let ref mut fresh1 = *((*stack).base).offset((*stack).size as isize);
    *fresh1 = item;
    let ref mut fresh2 = (*stack).size;
    *fresh2 = (*fresh2).wrapping_add(1);
    return 0 as libc::c_int;
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
unsafe extern "C" fn stack_pop(mut stack: *mut gv_stack_t) -> *mut libc::c_void {
    let mut top: *mut libc::c_void = stack_top(stack);
    let ref mut fresh3 = (*stack).size;
    *fresh3 = (*fresh3).wrapping_sub(1);
    return top;
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
unsafe extern "C" fn push(mut s: *mut gv_stack_t, mut x: Pair) {
    let mut copy: *mut Pair = gv_alloc(::std::mem::size_of::<Pair>() as libc::c_ulong)
        as *mut Pair;
    *copy = x;
    stack_push_or_exit(s, copy as *mut libc::c_void);
}
unsafe extern "C" fn pop(mut s: *mut gv_stack_t, mut x: *mut Pair) -> bool {
    if stack_is_empty(s) {
        return 0 as libc::c_int != 0;
    }
    let mut top: *mut Pair = stack_pop(s) as *mut Pair;
    *x = *top;
    free(top as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn heapify(mut h: *mut PairHeap, mut i: libc::c_int) {
    let mut l: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut largest: libc::c_int = 0;
    loop {
        l = 2 as libc::c_int * i;
        r = 2 as libc::c_int * i + 1 as libc::c_int;
        if l < (*h).heapSize
            && ((*((*h).data).offset(l as isize)).dist
                < (*((*h).data).offset(i as isize)).dist
                || (*((*h).data).offset(l as isize)).dist
                    == (*((*h).data).offset(i as isize)).dist
                    && rand() % 2 as libc::c_int != 0)
        {
            largest = l;
        } else {
            largest = i;
        }
        if r < (*h).heapSize
            && ((*((*h).data).offset(r as isize)).dist
                < (*((*h).data).offset(largest as isize)).dist
                || (*((*h).data).offset(r as isize)).dist
                    == (*((*h).data).offset(largest as isize)).dist
                    && rand() % 2 as libc::c_int != 0)
        {
            largest = r;
        }
        if largest == i {
            break;
        }
        let mut temp: Pair = Pair {
            left: 0,
            right: 0,
            dist: 0.,
        };
        temp = *((*h).data).offset(largest as isize);
        *((*h).data).offset(largest as isize) = *((*h).data).offset(i as isize);
        *((*h).data).offset(i as isize) = temp;
        i = largest;
    };
}
unsafe extern "C" fn freeHeap(mut h: *mut PairHeap) {
    free((*h).data as *mut libc::c_void);
}
unsafe extern "C" fn initHeap(
    mut h: *mut PairHeap,
    mut place: *mut libc::c_double,
    mut ordering: *mut libc::c_int,
    mut n: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut edge: Pair = Pair {
        left: 0,
        right: 0,
        dist: 0.,
    };
    let mut j: libc::c_int = 0;
    (*h).heapSize = n - 1 as libc::c_int;
    (*h).maxSize = (*h).heapSize;
    let ref mut fresh4 = (*h).data;
    *fresh4 = gcalloc(
        (*h).maxSize as size_t,
        ::std::mem::size_of::<Pair>() as libc::c_ulong,
    ) as *mut Pair;
    i = 0 as libc::c_int;
    while i < n - 1 as libc::c_int {
        edge.left = *ordering.offset(i as isize);
        edge.right = *ordering.offset((i + 1 as libc::c_int) as isize);
        edge
            .dist = *place
            .offset(*ordering.offset((i + 1 as libc::c_int) as isize) as isize)
            - *place.offset(*ordering.offset(i as isize) as isize);
        *((*h).data).offset(i as isize) = edge;
        i += 1;
    }
    j = (n - 1 as libc::c_int) / 2 as libc::c_int;
    while j >= 0 as libc::c_int {
        heapify(h, j);
        j -= 1;
    }
}
unsafe extern "C" fn extractMax(mut h: *mut PairHeap, mut max: *mut Pair) -> bool {
    if (*h).heapSize == 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    *max = *((*h).data).offset(0 as libc::c_int as isize);
    *((*h).data)
        .offset(
            0 as libc::c_int as isize,
        ) = *((*h).data).offset(((*h).heapSize - 1 as libc::c_int) as isize);
    let ref mut fresh5 = (*h).heapSize;
    *fresh5 -= 1;
    heapify(h, 0 as libc::c_int);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn insert(mut h: *mut PairHeap, mut edge: Pair) {
    let mut i: libc::c_int = (*h).heapSize;
    if (*h).heapSize == (*h).maxSize {
        (*h).maxSize *= 2 as libc::c_int;
        let ref mut fresh6 = (*h).data;
        *fresh6 = realloc(
            (*h).data as *mut libc::c_void,
            ((*h).maxSize as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<Pair>() as libc::c_ulong),
        ) as *mut Pair;
    }
    let ref mut fresh7 = (*h).heapSize;
    *fresh7 += 1;
    *((*h).data).offset(i as isize) = edge;
    while i > 0 as libc::c_int
        && ((*((*h).data).offset(i as isize)).dist
            < (*((*h).data).offset((i / 2 as libc::c_int) as isize)).dist
            || (*((*h).data).offset(i as isize)).dist
                == (*((*h).data).offset((i / 2 as libc::c_int) as isize)).dist
                && rand() % 2 as libc::c_int != 0)
    {
        let mut temp: Pair = Pair {
            left: 0,
            right: 0,
            dist: 0.,
        };
        temp = *((*h).data).offset(i as isize);
        *((*h).data)
            .offset(i as isize) = *((*h).data).offset((i / 2 as libc::c_int) as isize);
        *((*h).data).offset((i / 2 as libc::c_int) as isize) = temp;
        i = i / 2 as libc::c_int;
    }
}
unsafe extern "C" fn find_closest_pairs(
    mut place: *mut libc::c_double,
    mut n: libc::c_int,
    mut num_pairs: libc::c_int,
    mut pairs_stack: *mut gv_stack_t,
) {
    let mut i: libc::c_int = 0;
    let mut heap: PairHeap = PairHeap {
        data: 0 as *mut Pair,
        heapSize: 0,
        maxSize: 0,
    };
    let mut left: *mut libc::c_int = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut right: *mut libc::c_int = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut pair: Pair = {
        let mut init = Pair {
            left: 0 as libc::c_int,
            right: 0,
            dist: 0.,
        };
        init
    };
    let mut new_pair: Pair = Pair {
        left: 0,
        right: 0,
        dist: 0.,
    };
    let mut ordering: *mut libc::c_int = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut inv_ordering: *mut libc::c_int = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *ordering.offset(i as isize) = i;
        i += 1;
    }
    quicksort_place(place, ordering, 0 as libc::c_int, n - 1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < n {
        *inv_ordering.offset(*ordering.offset(i as isize) as isize) = i;
        i += 1;
    }
    initHeap(&mut heap, place, ordering, n);
    i = 1 as libc::c_int;
    while i < n {
        *left
            .offset(
                *ordering.offset(i as isize) as isize,
            ) = *ordering.offset((i - 1 as libc::c_int) as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n - 1 as libc::c_int {
        *right
            .offset(
                *ordering.offset(i as isize) as isize,
            ) = *ordering.offset((i + 1 as libc::c_int) as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < num_pairs {
        let mut left_index: libc::c_int = 0;
        let mut right_index: libc::c_int = 0;
        let mut neighbor: libc::c_int = 0;
        if !extractMax(&mut heap, &mut pair) {
            break;
        }
        push(pairs_stack, pair);
        left_index = *inv_ordering.offset(pair.left as isize);
        right_index = *inv_ordering.offset(pair.right as isize);
        if left_index > 0 as libc::c_int {
            neighbor = *ordering.offset((left_index - 1 as libc::c_int) as isize);
            if *inv_ordering.offset(*right.offset(neighbor as isize) as isize)
                < right_index
            {
                new_pair.left = neighbor;
                new_pair.right = pair.right;
                new_pair
                    .dist = *place.offset(pair.right as isize)
                    - *place.offset(neighbor as isize);
                insert(&mut heap, new_pair);
                *right.offset(neighbor as isize) = pair.right;
                *left.offset(pair.right as isize) = neighbor;
            }
        }
        if right_index < n - 1 as libc::c_int {
            neighbor = *ordering.offset((right_index + 1 as libc::c_int) as isize);
            if *inv_ordering.offset(*left.offset(neighbor as isize) as isize)
                > left_index
            {
                new_pair.left = pair.left;
                new_pair.right = neighbor;
                new_pair
                    .dist = *place.offset(neighbor as isize)
                    - *place.offset(pair.left as isize);
                insert(&mut heap, new_pair);
                *left.offset(neighbor as isize) = pair.left;
                *right.offset(pair.left as isize) = neighbor;
            }
        }
        i += 1;
    }
    free(left as *mut libc::c_void);
    free(right as *mut libc::c_void);
    free(ordering as *mut libc::c_void);
    free(inv_ordering as *mut libc::c_void);
    freeHeap(&mut heap);
}
unsafe extern "C" fn add_edge(
    mut graph: *mut vtx_data,
    mut u: libc::c_int,
    mut v: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*graph.offset(u as isize)).nedges {
        if *((*graph.offset(u as isize)).edges).offset(i as isize) == v {
            return;
        }
        i += 1;
    }
    let ref mut fresh8 = (*graph.offset(u as isize)).nedges;
    let fresh9 = *fresh8;
    *fresh8 = *fresh8 + 1;
    *((*graph.offset(u as isize)).edges).offset(fresh9 as isize) = v;
    let ref mut fresh10 = (*graph.offset(v as isize)).nedges;
    let fresh11 = *fresh10;
    *fresh10 = *fresh10 + 1;
    *((*graph.offset(v as isize)).edges).offset(fresh11 as isize) = u;
    if !((*graph.offset(0 as libc::c_int as isize)).ewgts).is_null() {
        let ref mut fresh12 = *((*graph.offset(u as isize)).ewgts)
            .offset(0 as libc::c_int as isize);
        *fresh12 -= 1.;
        let ref mut fresh13 = *((*graph.offset(v as isize)).ewgts)
            .offset(0 as libc::c_int as isize);
        *fresh13 -= 1.;
    }
}
unsafe extern "C" fn construct_graph(
    mut n: libc::c_int,
    mut edges_stack: *mut gv_stack_t,
    mut New_graph: *mut *mut vtx_data,
) {
    let mut new_graph: *mut vtx_data = 0 as *mut vtx_data;
    let mut degrees: *mut libc::c_int = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut top: size_t = stack_size(edges_stack);
    let mut new_nedges: size_t = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(top)
        .wrapping_add(n as libc::c_ulong);
    let mut pair: Pair = Pair {
        left: 0,
        right: 0,
        dist: 0.,
    };
    let mut edges: *mut libc::c_int = gcalloc(
        new_nedges,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut weights: *mut libc::c_float = gcalloc(
        new_nedges,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        *degrees.offset(i as isize) = 1 as libc::c_int;
        i += 1;
    }
    let mut i_0: size_t = 0 as libc::c_int as size_t;
    while i_0 < top {
        pair = *(*((*edges_stack).base).offset(i_0 as isize) as *mut Pair);
        let ref mut fresh14 = *degrees.offset(pair.left as isize);
        *fresh14 += 1;
        let ref mut fresh15 = *degrees.offset(pair.right as isize);
        *fresh15 += 1;
        i_0 = i_0.wrapping_add(1);
    }
    let mut i_1: size_t = 0 as libc::c_int as size_t;
    while i_1 < new_nedges {
        *weights.offset(i_1 as isize) = 1.0f64 as libc::c_float;
        i_1 = i_1.wrapping_add(1);
    }
    new_graph = gcalloc(n as size_t, ::std::mem::size_of::<vtx_data>() as libc::c_ulong)
        as *mut vtx_data;
    *New_graph = new_graph;
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < n {
        (*new_graph.offset(i_2 as isize)).nedges = 1 as libc::c_int;
        let ref mut fresh16 = (*new_graph.offset(i_2 as isize)).ewgts;
        *fresh16 = weights;
        let ref mut fresh17 = (*new_graph.offset(i_2 as isize)).edges;
        *fresh17 = edges;
        *edges = i_2;
        *weights = 0 as libc::c_int as libc::c_float;
        weights = weights.offset(*degrees.offset(i_2 as isize) as isize);
        edges = edges.offset(*degrees.offset(i_2 as isize) as isize);
        i_2 += 1;
    }
    free(degrees as *mut libc::c_void);
    while pop(edges_stack, &mut pair) {
        add_edge(new_graph, pair.left, pair.right);
    }
}
#[no_mangle]
pub unsafe extern "C" fn closest_pairs2graph(
    mut place: *mut libc::c_double,
    mut n: libc::c_int,
    mut num_pairs: libc::c_int,
    mut graph: *mut *mut vtx_data,
) {
    let mut pairs_stack: gv_stack_t = {
        let mut init = gv_stack_t {
            base: 0 as *mut *mut libc::c_void,
            size: 0,
            capacity: 0,
        };
        init
    };
    find_closest_pairs(place, n, num_pairs, &mut pairs_stack);
    construct_graph(n, &mut pairs_stack, graph);
    stack_reset(&mut pairs_stack);
}
