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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slist {
    pub next: *mut slist,
    pub userdata: *const libc::c_void,
}
pub type SList = slist;
pub type SListCallback = unsafe extern "C" fn(*mut SList, *mut libc::c_void) -> *mut libc::c_void;
pub type SListCompare =
    unsafe extern "C" fn(*const SList, *const SList, *mut libc::c_void) -> libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn lt__slist_delete(
    mut head: *mut SList,
    mut delete_fct: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) -> *mut SList {
    if delete_fct.is_some() {
    } else {
        __assert_fail(
            b"delete_fct\0" as *const u8 as *const libc::c_char,
            b"slist.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"SList *lt__slist_delete(SList *, void (*)(void *))\0",
            ))
            .as_ptr(),
        );
    }
    while !head.is_null() {
        let mut next: *mut SList = (*head).next;
        (Some(delete_fct.expect("non-null function pointer"))).expect("non-null function pointer")(
            head as *mut libc::c_void,
        );
        head = next;
    }
    return 0 as *mut SList;
}
#[no_mangle]
pub unsafe extern "C" fn lt__slist_remove(
    mut phead: *mut *mut SList,
    mut find: Option<SListCallback>,
    mut matchdata: *mut libc::c_void,
) -> *mut SList {
    let mut stale: *mut SList = 0 as *mut SList;
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    if find.is_some() {
    } else {
        __assert_fail(
            b"find\0" as *const u8 as *const libc::c_char,
            b"slist.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                b"SList *lt__slist_remove(SList **, SListCallback *, void *)\0",
            ))
            .as_ptr(),
        );
    }
    if phead.is_null() || (*phead).is_null() {
        return 0 as *mut SList;
    }
    result = (Some(find.expect("non-null function pointer"))).expect("non-null function pointer")(
        *phead, matchdata,
    );
    if !result.is_null() {
        stale = *phead;
        *phead = (*stale).next;
    } else {
        let mut head: *mut SList = 0 as *mut SList;
        head = *phead;
        while !((*head).next).is_null() {
            result = (Some(find.expect("non-null function pointer")))
                .expect("non-null function pointer")((*head).next, matchdata);
            if !result.is_null() {
                stale = (*head).next;
                let ref mut fresh0 = (*head).next;
                *fresh0 = (*stale).next;
                break;
            } else {
                head = (*head).next;
            }
        }
    }
    return result as *mut SList;
}
#[no_mangle]
pub unsafe extern "C" fn lt__slist_find(
    mut slist: *mut SList,
    mut find: Option<SListCallback>,
    mut matchdata: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    if find.is_some() {
    } else {
        __assert_fail(
            b"find\0" as *const u8 as *const libc::c_char,
            b"slist.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                b"void *lt__slist_find(SList *, SListCallback *, void *)\0",
            ))
            .as_ptr(),
        );
    }
    while !slist.is_null() {
        result = (Some(find.expect("non-null function pointer")))
            .expect("non-null function pointer")(slist, matchdata);
        if !result.is_null() {
            break;
        }
        slist = (*slist).next;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn lt__slist_concat(
    mut head: *mut SList,
    mut tail: *mut SList,
) -> *mut SList {
    let mut last: *mut SList = 0 as *mut SList;
    if head.is_null() {
        return tail;
    }
    last = head;
    while !((*last).next).is_null() {
        last = (*last).next;
    }
    let ref mut fresh1 = (*last).next;
    *fresh1 = tail;
    return head;
}
#[no_mangle]
pub unsafe extern "C" fn lt__slist_cons(mut item: *mut SList, mut slist: *mut SList) -> *mut SList {
    if item.is_null() {
        return slist;
    }
    if ((*item).next).is_null() {
    } else {
        __assert_fail(
            b"!item->next\0" as *const u8 as *const libc::c_char,
            b"slist.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"SList *lt__slist_cons(SList *, SList *)\0",
            ))
            .as_ptr(),
        );
    }
    let ref mut fresh2 = (*item).next;
    *fresh2 = slist;
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn lt__slist_tail(mut slist: *mut SList) -> *mut SList {
    return if !slist.is_null() {
        (*slist).next
    } else {
        0 as *mut slist
    };
}
#[no_mangle]
pub unsafe extern "C" fn lt__slist_nth(mut slist: *mut SList, mut n: size_t) -> *mut SList {
    while n > 1 as libc::c_int as libc::c_ulong && !slist.is_null() {
        slist = (*slist).next;
        n = n.wrapping_sub(1);
    }
    return slist;
}
#[no_mangle]
pub unsafe extern "C" fn lt__slist_length(mut slist: *mut SList) -> size_t {
    let mut n: size_t = 0;
    n = 0 as libc::c_int as size_t;
    while !slist.is_null() {
        slist = (*slist).next;
        n = n.wrapping_add(1);
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn lt__slist_reverse(mut slist: *mut SList) -> *mut SList {
    let mut result: *mut SList = 0 as *mut SList;
    let mut next: *mut SList = 0 as *mut SList;
    while !slist.is_null() {
        next = (*slist).next;
        let ref mut fresh3 = (*slist).next;
        *fresh3 = result;
        result = slist;
        slist = next;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn lt__slist_foreach(
    mut slist: *mut SList,
    mut foreach: Option<SListCallback>,
    mut userdata: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    if foreach.is_some() {
    } else {
        __assert_fail(
            b"foreach\0" as *const u8 as *const libc::c_char,
            b"slist.c\0" as *const u8 as *const libc::c_char,
            246 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                b"void *lt__slist_foreach(SList *, SListCallback *, void *)\0",
            ))
            .as_ptr(),
        );
    }
    while !slist.is_null() {
        let mut next: *mut SList = (*slist).next;
        result = (Some(foreach.expect("non-null function pointer")))
            .expect("non-null function pointer")(slist, userdata);
        if !result.is_null() {
            break;
        }
        slist = next;
    }
    return result;
}
unsafe extern "C" fn slist_sort_merge(
    mut left: *mut SList,
    mut right: *mut SList,
    mut compare: Option<SListCompare>,
    mut userdata: *mut libc::c_void,
) -> *mut SList {
    let mut merged: SList = SList {
        next: 0 as *mut slist,
        userdata: 0 as *const libc::c_void,
    };
    let mut insert: *mut SList = 0 as *mut SList;
    insert = &mut merged;
    while !left.is_null() && !right.is_null() {
        if (Some(compare.expect("non-null function pointer"))).expect("non-null function pointer")(
            left, right, userdata,
        ) <= 0 as libc::c_int
        {
            let ref mut fresh4 = (*insert).next;
            *fresh4 = left;
            insert = *fresh4;
            left = (*left).next;
        } else {
            let ref mut fresh5 = (*insert).next;
            *fresh5 = right;
            insert = *fresh5;
            right = (*right).next;
        }
    }
    let ref mut fresh6 = (*insert).next;
    *fresh6 = if !left.is_null() { left } else { right };
    return merged.next;
}
#[no_mangle]
pub unsafe extern "C" fn lt__slist_sort(
    mut slist: *mut SList,
    mut compare: Option<SListCompare>,
    mut userdata: *mut libc::c_void,
) -> *mut SList {
    let mut left: *mut SList = 0 as *mut SList;
    let mut right: *mut SList = 0 as *mut SList;
    if slist.is_null() {
        return slist;
    }
    left = slist;
    right = (*slist).next;
    if right.is_null() {
        return left;
    }
    while !right.is_null() && {
        right = (*right).next;
        !right.is_null()
    } {
        if right.is_null() || {
            right = (*right).next;
            right.is_null()
        } {
            break;
        }
        slist = (*slist).next;
    }
    right = (*slist).next;
    let ref mut fresh7 = (*slist).next;
    *fresh7 = 0 as *mut slist;
    return slist_sort_merge(
        lt__slist_sort(left, compare, userdata),
        lt__slist_sort(right, compare, userdata),
        compare,
        userdata,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lt__slist_box(mut userdata: *const libc::c_void) -> *mut SList {
    let mut item: *mut SList =
        malloc(::std::mem::size_of::<SList>() as libc::c_ulong) as *mut SList;
    if !item.is_null() {
        let ref mut fresh8 = (*item).next;
        *fresh8 = 0 as *mut slist;
        let ref mut fresh9 = (*item).userdata;
        *fresh9 = userdata;
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn lt__slist_unbox(mut item: *mut SList) -> *mut libc::c_void {
    let mut userdata: *mut libc::c_void = 0 as *mut libc::c_void;
    if !item.is_null() {
        userdata = (*item).userdata as *mut libc::c_void;
        free(item as *mut libc::c_void);
    }
    return userdata;
}
