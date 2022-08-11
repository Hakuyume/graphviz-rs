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
#![feature(label_break_value, register_tool)]
extern "C" {
    fn DoubleLinkedList_new(data: *mut libc::c_void) -> DoubleLinkedList;
    fn DoubleLinkedList_delete(
        head: DoubleLinkedList,
        linklist_deallocator: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn DoubleLinkedList_prepend(l: DoubleLinkedList, data: *mut libc::c_void) -> DoubleLinkedList;
    fn DoubleLinkedList_get_data(l: DoubleLinkedList) -> *mut libc::c_void;
    fn DoubleLinkedList_delete_element(
        l: DoubleLinkedList,
        linklist_deallocator: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        head: *mut DoubleLinkedList,
    );
    fn free(_: *mut libc::c_void);
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DoubleLinkedList_struct {
    pub data: *mut libc::c_void,
    pub next: DoubleLinkedList,
    pub prev: DoubleLinkedList,
}
pub type DoubleLinkedList = *mut DoubleLinkedList_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PriorityQueue_struct {
    pub count: libc::c_int,
    pub n: libc::c_int,
    pub ngain: libc::c_int,
    pub gain_max: libc::c_int,
    pub buckets: *mut DoubleLinkedList,
    pub where_0: *mut DoubleLinkedList,
    pub gain: *mut libc::c_int,
}
pub type PriorityQueue = *mut PriorityQueue_struct;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn PriorityQueue_new(
    mut n: libc::c_int,
    mut ngain: libc::c_int,
) -> PriorityQueue {
    let mut q: PriorityQueue = 0 as *mut PriorityQueue_struct;
    let mut i: libc::c_int = 0;
    q = gcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<PriorityQueue_struct>() as libc::c_ulong,
    ) as *mut PriorityQueue_struct;
    (*q).count = 0 as libc::c_int;
    (*q).n = n;
    (*q).ngain = ngain;
    (*q).gain_max = -(1 as libc::c_int);
    let ref mut fresh0 = (*q).buckets;
    *fresh0 = gcalloc(
        (ngain + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<DoubleLinkedList>() as libc::c_ulong,
    ) as *mut DoubleLinkedList;
    i = 0 as libc::c_int;
    while i < ngain + 1 as libc::c_int {
        let ref mut fresh1 = *((*q).buckets).offset(i as isize);
        *fresh1 = 0 as DoubleLinkedList;
        i += 1;
    }
    let ref mut fresh2 = (*q).where_0;
    *fresh2 = gcalloc(
        (n + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<DoubleLinkedList>() as libc::c_ulong,
    ) as *mut DoubleLinkedList;
    i = 0 as libc::c_int;
    while i < n + 1 as libc::c_int {
        let ref mut fresh3 = *((*q).where_0).offset(i as isize);
        *fresh3 = 0 as DoubleLinkedList;
        i += 1;
    }
    let ref mut fresh4 = (*q).gain;
    *fresh4 = gcalloc(
        (n + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n + 1 as libc::c_int {
        *((*q).gain).offset(i as isize) = -(999 as libc::c_int);
        i += 1;
    }
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn PriorityQueue_delete(mut q: PriorityQueue) {
    let mut i: libc::c_int = 0;
    if !q.is_null() {
        if !((*q).buckets).is_null() {
            i = 0 as libc::c_int;
            while i < (*q).ngain + 1 as libc::c_int {
                DoubleLinkedList_delete(
                    *((*q).buckets).offset(i as isize),
                    Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                );
                i += 1;
            }
            free((*q).buckets as *mut libc::c_void);
        }
        free((*q).where_0 as *mut libc::c_void);
        free((*q).gain as *mut libc::c_void);
        free(q as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PriorityQueue_push(
    mut q: PriorityQueue,
    mut i: libc::c_int,
    mut gain: libc::c_int,
) -> PriorityQueue {
    let mut l: DoubleLinkedList = 0 as *mut DoubleLinkedList_struct;
    let mut data: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut gainold: libc::c_int = 0;
    if !q.is_null() {
    } else {
        __assert_fail(
            b"q\0" as *const u8 as *const libc::c_char,
            b"PriorityQueue.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                b"PriorityQueue PriorityQueue_push(PriorityQueue, int, int)\0",
            ))
            .as_ptr(),
        );
    }
    if gain <= (*q).ngain {
    } else {
        __assert_fail(
            b"gain <= q->ngain\0" as *const u8 as *const libc::c_char,
            b"PriorityQueue.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                b"PriorityQueue PriorityQueue_push(PriorityQueue, int, int)\0",
            ))
            .as_ptr(),
        );
    }
    if (*((*q).where_0).offset(i as isize)).is_null() {
        let ref mut fresh5 = (*q).count;
        *fresh5 += 1;
        if gain > (*q).gain_max {
            (*q).gain_max = gain;
        }
        *((*q).gain).offset(i as isize) = gain;
        data = gcalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        *data.offset(0 as libc::c_int as isize) = i;
        l = *((*q).buckets).offset(gain as isize);
        if !l.is_null() {
            let ref mut fresh6 = *((*q).where_0).offset(i as isize);
            *fresh6 = DoubleLinkedList_prepend(l, data as *mut libc::c_void);
            let ref mut fresh7 = *((*q).buckets).offset(gain as isize);
            *fresh7 = *fresh6;
        } else {
            let ref mut fresh8 = *((*q).where_0).offset(i as isize);
            *fresh8 = DoubleLinkedList_new(data as *mut libc::c_void);
            let ref mut fresh9 = *((*q).buckets).offset(gain as isize);
            *fresh9 = *fresh8;
        }
    } else {
        l = *((*q).where_0).offset(i as isize);
        gainold = *((*q).gain).offset(i as isize);
        let ref mut fresh10 = *((*q).where_0).offset(i as isize);
        *fresh10 = 0 as DoubleLinkedList;
        let ref mut fresh11 = (*q).count;
        *fresh11 -= 1;
        DoubleLinkedList_delete_element(
            l,
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut *((*q).buckets).offset(gainold as isize),
        );
        return PriorityQueue_push(q, i, gain);
    }
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn PriorityQueue_pop(
    mut q: PriorityQueue,
    mut i: *mut libc::c_int,
    mut gain: *mut libc::c_int,
) -> libc::c_int {
    let mut gain_max: libc::c_int = 0;
    let mut l: DoubleLinkedList = 0 as *mut DoubleLinkedList_struct;
    let mut data: *mut libc::c_int = 0 as *mut libc::c_int;
    if q.is_null() || (*q).count <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    gain_max = (*q).gain_max;
    *gain = gain_max;
    let ref mut fresh12 = (*q).count;
    *fresh12 -= 1;
    l = *((*q).buckets).offset(gain_max as isize);
    data = DoubleLinkedList_get_data(l) as *mut libc::c_int;
    *i = *data.offset(0 as libc::c_int as isize);
    DoubleLinkedList_delete_element(
        l,
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut *((*q).buckets).offset(gain_max as isize),
    );
    if (*((*q).buckets).offset(gain_max as isize)).is_null() {
        while gain_max >= 0 as libc::c_int && (*((*q).buckets).offset(gain_max as isize)).is_null()
        {
            gain_max -= 1;
        }
        (*q).gain_max = gain_max;
    }
    let ref mut fresh13 = *((*q).where_0).offset(*i as isize);
    *fresh13 = 0 as DoubleLinkedList;
    *((*q).gain).offset(*i as isize) = -(999 as libc::c_int);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PriorityQueue_get_gain(
    mut q: PriorityQueue,
    mut i: libc::c_int,
) -> libc::c_int {
    return *((*q).gain).offset(i as isize);
}
#[no_mangle]
pub unsafe extern "C" fn PriorityQueue_remove(
    mut q: PriorityQueue,
    mut i: libc::c_int,
) -> libc::c_int {
    let mut gain: libc::c_int = 0;
    let mut gain_max: libc::c_int = 0;
    let mut l: DoubleLinkedList = 0 as *mut DoubleLinkedList_struct;
    if q.is_null() || (*q).count <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    gain = *((*q).gain).offset(i as isize);
    let ref mut fresh14 = (*q).count;
    *fresh14 -= 1;
    l = *((*q).where_0).offset(i as isize);
    DoubleLinkedList_delete_element(
        l,
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut *((*q).buckets).offset(gain as isize),
    );
    gain_max = (*q).gain_max;
    if gain == gain_max && (*((*q).buckets).offset(gain_max as isize)).is_null() {
        while gain_max >= 0 as libc::c_int && (*((*q).buckets).offset(gain_max as isize)).is_null()
        {
            gain_max -= 1;
        }
        (*q).gain_max = gain_max;
    }
    let ref mut fresh15 = *((*q).where_0).offset(i as isize);
    *fresh15 = 0 as DoubleLinkedList;
    *((*q).gain).offset(i as isize) = -(999 as libc::c_int);
    return 1 as libc::c_int;
}
