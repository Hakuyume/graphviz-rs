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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn IntStack_pop(s: IntStack, flag: *mut libc::c_int) -> libc::c_int;
    fn IntStack_push(s: IntStack, i: libc::c_int) -> size_t;
    fn IntStack_delete(s: IntStack);
    fn IntStack_new() -> IntStack;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
pub struct IntStack_struct {
    pub last: size_t,
    pub max_len: size_t,
    pub stack: *mut libc::c_int,
}
pub type IntStack = *mut IntStack_struct;
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
pub type C2RustUnnamed = libc::c_int;
pub const BinaryHeap_error_malloc: C2RustUnnamed = -10;
pub type BinaryHeap = *mut BinaryHeap_struct;
#[no_mangle]
pub unsafe extern "C" fn BinaryHeap_new(
    mut cmp: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int>,
) -> BinaryHeap {
    let mut h: BinaryHeap = 0 as *mut BinaryHeap_struct;
    let mut max_len: size_t = ((1 as libc::c_int) << 8 as libc::c_int) as size_t;
    h = gmalloc(::std::mem::size_of::<BinaryHeap_struct>() as libc::c_ulong) as BinaryHeap;
    (*h).max_len = max_len;
    (*h).len = 0 as libc::c_int as size_t;
    let ref mut fresh0 = (*h).heap;
    *fresh0 = gcalloc(
        max_len,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
    ) as *mut *mut libc::c_void;
    let ref mut fresh1 = (*h).id_to_pos;
    *fresh1 = gcalloc(max_len, ::std::mem::size_of::<size_t>() as libc::c_ulong) as *mut size_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < max_len {
        *((*h).id_to_pos).offset(i as isize) = 18446744073709551615 as libc::c_ulong;
        i = i.wrapping_add(1);
    }
    let ref mut fresh2 = (*h).pos_to_id;
    *fresh2 = gcalloc(
        max_len,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let ref mut fresh3 = (*h).id_stack;
    *fresh3 = IntStack_new();
    let ref mut fresh4 = (*h).cmp;
    *fresh4 = cmp;
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn BinaryHeap_delete(
    mut h: BinaryHeap,
    mut del: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    if h.is_null() {
        return;
    }
    free((*h).id_to_pos as *mut libc::c_void);
    free((*h).pos_to_id as *mut libc::c_void);
    IntStack_delete((*h).id_stack);
    if del.is_some() {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < (*h).len {
            del.expect("non-null function pointer")(*((*h).heap).offset(i as isize));
            i = i.wrapping_add(1);
        }
    }
    free((*h).heap as *mut libc::c_void);
    free(h as *mut libc::c_void);
}
unsafe extern "C" fn BinaryHeap_realloc(mut h: BinaryHeap) -> BinaryHeap {
    let mut max_len0: size_t = (*h).max_len;
    let mut max_len: size_t = (*h).max_len;
    let mut i: size_t = 0;
    max_len = max_len.wrapping_add(
        (if max_len.wrapping_div(5 as libc::c_int as libc::c_ulong)
            > 10 as libc::c_int as libc::c_ulong
        {
            max_len.wrapping_div(5 as libc::c_int as libc::c_ulong)
        } else {
            10 as libc::c_int as libc::c_ulong
        }),
    );
    (*h).max_len = max_len;
    let ref mut fresh5 = (*h).heap;
    *fresh5 = grealloc(
        (*h).heap as *mut libc::c_void,
        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong).wrapping_mul(max_len),
    ) as *mut *mut libc::c_void;
    if ((*h).heap).is_null() {
        return 0 as BinaryHeap;
    }
    let ref mut fresh6 = (*h).id_to_pos;
    *fresh6 = grealloc(
        (*h).id_to_pos as *mut libc::c_void,
        (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(max_len),
    ) as *mut size_t;
    if ((*h).id_to_pos).is_null() {
        return 0 as BinaryHeap;
    }
    let ref mut fresh7 = (*h).pos_to_id;
    *fresh7 = grealloc(
        (*h).pos_to_id as *mut libc::c_void,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(max_len),
    ) as *mut libc::c_int;
    if ((*h).pos_to_id).is_null() {
        return 0 as BinaryHeap;
    }
    i = max_len0;
    while i < max_len {
        *((*h).id_to_pos).offset(i as isize) = 18446744073709551615 as libc::c_ulong;
        i = i.wrapping_add(1);
    }
    return h;
}
unsafe extern "C" fn swap(mut h: BinaryHeap, mut parentPos: size_t, mut nodePos: size_t) {
    let mut parentID: libc::c_int = 0;
    let mut nodeID: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut heap: *mut *mut libc::c_void = (*h).heap;
    let mut id_to_pos: *mut size_t = (*h).id_to_pos;
    let mut pos_to_id: *mut libc::c_int = (*h).pos_to_id;
    if parentPos < (*h).len {
    } else {
        __assert_fail(
            b"parentPos < h->len\0" as *const u8 as *const libc::c_char,
            b"BinaryHeap.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void swap(BinaryHeap, size_t, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    if nodePos < (*h).len {
    } else {
        __assert_fail(
            b"nodePos < h->len\0" as *const u8 as *const libc::c_char,
            b"BinaryHeap.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void swap(BinaryHeap, size_t, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    parentID = *pos_to_id.offset(parentPos as isize);
    nodeID = *pos_to_id.offset(nodePos as isize);
    tmp = *heap.offset(parentPos as isize);
    let ref mut fresh8 = *heap.offset(parentPos as isize);
    *fresh8 = *heap.offset(nodePos as isize);
    let ref mut fresh9 = *heap.offset(nodePos as isize);
    *fresh9 = tmp;
    *pos_to_id.offset(parentPos as isize) = nodeID;
    *id_to_pos.offset(nodeID as isize) = parentPos;
    *pos_to_id.offset(nodePos as isize) = parentID;
    *id_to_pos.offset(parentID as isize) = nodePos;
}
unsafe extern "C" fn siftUp(mut h: BinaryHeap, mut nodePos: size_t) -> size_t {
    let mut heap: *mut *mut libc::c_void = (*h).heap;
    if nodePos != 0 as libc::c_int as libc::c_ulong {
        let mut parentPos: size_t = nodePos
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        if ((*h).cmp).expect("non-null function pointer")(
            *heap.offset(parentPos as isize),
            *heap.offset(nodePos as isize),
        ) == 1 as libc::c_int
        {
            swap(h, parentPos, nodePos);
            nodePos = siftUp(h, parentPos);
        }
    }
    return nodePos;
}
unsafe extern "C" fn siftDown(mut h: BinaryHeap, mut nodePos: size_t) -> size_t {
    let mut childPos: size_t = 0;
    let mut heap: *mut *mut libc::c_void = (*h).heap;
    let mut childPos1: size_t = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(nodePos)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut childPos2: size_t = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(nodePos)
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
    if (*h).len > 0 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(
            b"h->len > 0\0" as *const u8 as *const libc::c_char,
            b"BinaryHeap.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                b"size_t siftDown(BinaryHeap, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    if childPos1 > ((*h).len).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        return nodePos;
    }
    if childPos1 == ((*h).len).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        childPos = childPos1;
    } else if ((*h).cmp).expect("non-null function pointer")(
        *heap.offset(childPos1 as isize),
        *heap.offset(childPos2 as isize),
    ) == 1 as libc::c_int
    {
        childPos = childPos2;
    } else {
        childPos = childPos1;
    }
    if ((*h).cmp).expect("non-null function pointer")(
        *heap.offset(nodePos as isize),
        *heap.offset(childPos as isize),
    ) == 1 as libc::c_int
    {
        swap(h, nodePos, childPos);
        nodePos = siftDown(h, childPos);
    }
    return nodePos;
}
#[no_mangle]
pub unsafe extern "C" fn BinaryHeap_insert(
    mut h: BinaryHeap,
    mut item: *mut libc::c_void,
) -> libc::c_int {
    let mut len: size_t = (*h).len;
    if len <= 2147483647 as libc::c_int as size_t {
    } else {
        __assert_fail(
            b"len <= (size_t)INT_MAX\0" as *const u8 as *const libc::c_char,
            b"BinaryHeap.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"int BinaryHeap_insert(BinaryHeap, void *)\0",
            ))
            .as_ptr(),
        );
    }
    let mut id: libc::c_int = len as libc::c_int;
    let mut flag: libc::c_int = 0;
    if len > ((*h).max_len).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        if (BinaryHeap_realloc(h)).is_null() {
            return BinaryHeap_error_malloc as libc::c_int;
        }
    }
    id = IntStack_pop((*h).id_stack, &mut flag);
    if flag != 0 {
        id = len as libc::c_int;
    }
    let ref mut fresh10 = *((*h).heap).offset(len as isize);
    *fresh10 = item;
    *((*h).id_to_pos).offset(id as isize) = len;
    *((*h).pos_to_id).offset(len as isize) = id;
    let ref mut fresh11 = (*h).len;
    *fresh11 = (*fresh11).wrapping_add(1);
    let mut pos: size_t = siftUp(h, len);
    if *((*h).id_to_pos).offset(id as isize) == pos {
    } else {
        __assert_fail(
            b"h->id_to_pos[id] == pos\0" as *const u8 as *const libc::c_char,
            b"BinaryHeap.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"int BinaryHeap_insert(BinaryHeap, void *)\0",
            ))
            .as_ptr(),
        );
    }
    if *((*h).pos_to_id).offset(pos as isize) == id {
    } else {
        __assert_fail(
            b"h->pos_to_id[pos] == id\0" as *const u8 as *const libc::c_char,
            b"BinaryHeap.c\0" as *const u8 as *const libc::c_char,
            159 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"int BinaryHeap_insert(BinaryHeap, void *)\0",
            ))
            .as_ptr(),
        );
    }
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn BinaryHeap_get_min(mut h: BinaryHeap) -> *mut libc::c_void {
    return *((*h).heap).offset(0 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn BinaryHeap_extract_min(mut h: BinaryHeap) -> *mut libc::c_void {
    if (*h).len == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    return BinaryHeap_extract_item(h, *((*h).pos_to_id).offset(0 as libc::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn BinaryHeap_extract_item(
    mut h: BinaryHeap,
    mut id: libc::c_int,
) -> *mut libc::c_void {
    let mut item: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut id_to_pos: *mut size_t = (*h).id_to_pos;
    if id >= 0 as libc::c_int && id as size_t >= (*h).max_len {
        return 0 as *mut libc::c_void;
    }
    let mut pos: size_t = *id_to_pos.offset(id as isize);
    if pos == 18446744073709551615 as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    if pos < (*h).len {
    } else {
        __assert_fail(
            b"pos < h->len\0" as *const u8 as *const libc::c_char,
            b"BinaryHeap.c\0" as *const u8 as *const libc::c_char,
            188 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"void *BinaryHeap_extract_item(BinaryHeap, int)\0",
            ))
            .as_ptr(),
        );
    }
    item = *((*h).heap).offset(pos as isize);
    IntStack_push((*h).id_stack, id);
    if pos < ((*h).len).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        swap(
            h,
            pos,
            ((*h).len).wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        let ref mut fresh12 = (*h).len;
        *fresh12 = (*fresh12).wrapping_sub(1);
        pos = siftUp(h, pos);
        pos = siftDown(h, pos);
    } else {
        let ref mut fresh13 = (*h).len;
        *fresh13 = (*fresh13).wrapping_sub(1);
    }
    *((*h).id_to_pos).offset(id as isize) = 18446744073709551615 as libc::c_ulong;
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn BinaryHeap_reset(
    mut h: BinaryHeap,
    mut id: libc::c_int,
    mut item: *mut libc::c_void,
) -> size_t {
    if id >= 0 as libc::c_int && id as size_t >= (*h).max_len {
        return 18446744073709551615 as libc::c_ulong;
    }
    let mut pos: size_t = *((*h).id_to_pos).offset(id as isize);
    if pos == 18446744073709551615 as libc::c_ulong {
        return 18446744073709551615 as libc::c_ulong;
    }
    let ref mut fresh14 = *((*h).heap).offset(pos as isize);
    *fresh14 = item;
    pos = siftUp(h, pos);
    pos = siftDown(h, pos);
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn BinaryHeap_get_item(
    mut h: BinaryHeap,
    mut id: libc::c_int,
) -> *mut libc::c_void {
    if id >= 0 as libc::c_int && id as size_t >= (*h).max_len {
        return 0 as *mut libc::c_void;
    }
    let mut pos: size_t = *((*h).id_to_pos).offset(id as isize);
    if pos == 18446744073709551615 as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    return *((*h).heap).offset(pos as isize);
}
#[no_mangle]
pub unsafe extern "C" fn BinaryHeap_sanity_check(mut h: BinaryHeap) {
    let mut pos_to_id: *mut libc::c_int = (*h).pos_to_id;
    let mut id_to_pos: *mut size_t = (*h).id_to_pos;
    let mut heap: *mut *mut libc::c_void = (*h).heap;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: size_t = 1 as libc::c_int as size_t;
    while i < (*h).len {
        let mut parentPos: size_t = i
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        if ((*h).cmp).expect("non-null function pointer")(
            *heap.offset(i as isize),
            *heap.offset(parentPos as isize),
        ) >= 0 as libc::c_int
        {
        } else {
            __assert_fail(
                b"(h->cmp)(heap[i], heap[parentPos]) >= 0\0" as *const u8 as *const libc::c_char,
                b"BinaryHeap.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"void BinaryHeap_sanity_check(BinaryHeap)\0",
                ))
                .as_ptr(),
            );
        }
        i = i.wrapping_add(1);
    }
    mask = gcalloc(
        ((*h).len)
            .wrapping_add((1 as libc::c_int as libc::c_ulong).wrapping_add((*(*h).id_stack).last)),
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut i_0: size_t = 0 as libc::c_int as size_t;
    while i_0 <= (*(*h).id_stack).last {
        let mut key_spare: libc::c_int = *((*(*h).id_stack).stack).offset(i_0 as isize);
        if *((*h).id_to_pos).offset(key_spare as isize) == 18446744073709551615 as libc::c_ulong {
        } else {
            __assert_fail(
                b"h->id_to_pos[key_spare] == SIZE_MAX\0" as *const u8 as *const libc::c_char,
                b"BinaryHeap.c\0" as *const u8 as *const libc::c_char,
                253 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"void BinaryHeap_sanity_check(BinaryHeap)\0",
                ))
                .as_ptr(),
            );
        }
        *mask.offset(key_spare as isize) = 1 as libc::c_int;
        i_0 = i_0.wrapping_add(1);
    }
    let mut i_1: size_t = 1 as libc::c_int as size_t;
    while i_1 < (*h).len {
        if *mask.offset(*pos_to_id.offset(i_1 as isize) as isize) == 0 as libc::c_int {
        } else {
            __assert_fail(
                b"mask[pos_to_id[i]] == 0\0" as *const u8 as *const libc::c_char,
                b"BinaryHeap.c\0" as *const u8 as *const libc::c_char,
                262 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"void BinaryHeap_sanity_check(BinaryHeap)\0",
                ))
                .as_ptr(),
            );
        }
        *mask.offset(*pos_to_id.offset(i_1 as isize) as isize) = 1 as libc::c_int;
        if *id_to_pos.offset(*pos_to_id.offset(i_1 as isize) as isize) == i_1 {
        } else {
            __assert_fail(
                b"id_to_pos[pos_to_id[i]] == i\0" as *const u8 as *const libc::c_char,
                b"BinaryHeap.c\0" as *const u8 as *const libc::c_char,
                264 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"void BinaryHeap_sanity_check(BinaryHeap)\0",
                ))
                .as_ptr(),
            );
        }
        i_1 = i_1.wrapping_add(1);
    }
    let mut i_2: size_t = 0 as libc::c_int as size_t;
    while i_2
        < ((*h).len)
            .wrapping_add((1 as libc::c_int as libc::c_ulong).wrapping_add((*(*h).id_stack).last))
    {
        if *mask.offset(i_2 as isize) != 0 as libc::c_int {
        } else {
            __assert_fail(
                b"mask[i] != 0\0" as *const u8 as *const libc::c_char,
                b"BinaryHeap.c\0" as *const u8 as *const libc::c_char,
                269 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"void BinaryHeap_sanity_check(BinaryHeap)\0",
                ))
                .as_ptr(),
            );
        }
        i_2 = i_2.wrapping_add(1);
    }
    free(mask as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn BinaryHeap_print(
    mut h: BinaryHeap,
    mut pnt: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    let mut k: size_t = 2 as libc::c_int as size_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*h).len {
        pnt.expect("non-null function pointer")(*((*h).heap).offset(i as isize));
        fprintf(
            stderr,
            b"(%d) \0" as *const u8 as *const libc::c_char,
            *((*h).pos_to_id).offset(i as isize),
        );
        if i == k.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            k = (k as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
        i = i.wrapping_add(1);
    }
    fprintf(
        stderr,
        b"\nSpare keys =\0" as *const u8 as *const libc::c_char,
    );
    let mut i_0: size_t = 0 as libc::c_int as size_t;
    while i_0 <= (*(*h).id_stack).last {
        fprintf(
            stderr,
            b"%d(%zu) \0" as *const u8 as *const libc::c_char,
            *((*(*h).id_stack).stack).offset(i_0 as isize),
            *((*h).id_to_pos).offset(*((*(*h).id_stack).stack).offset(i_0 as isize) as isize),
        );
        i_0 = i_0.wrapping_add(1);
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
