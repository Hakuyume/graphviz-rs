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
#![feature(register_tool)]
extern "C" {
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SingleLinkedList_struct {
    pub data: *mut libc::c_void,
    pub next: SingleLinkedList,
}
pub type SingleLinkedList = *mut SingleLinkedList_struct;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DoubleLinkedList_struct {
    pub data: *mut libc::c_void,
    pub next: DoubleLinkedList,
    pub prev: DoubleLinkedList,
}
pub type DoubleLinkedList = *mut DoubleLinkedList_struct;
#[no_mangle]
pub unsafe extern "C" fn SingleLinkedList_new(mut data: *mut libc::c_void) -> SingleLinkedList {
    let mut head: SingleLinkedList = 0 as *mut SingleLinkedList_struct;
    head = gmalloc(::std::mem::size_of::<SingleLinkedList_struct>() as libc::c_ulong)
        as *mut SingleLinkedList_struct;
    let ref mut fresh0 = (*head).data;
    *fresh0 = data;
    let ref mut fresh1 = (*head).next;
    *fresh1 = 0 as SingleLinkedList;
    return head;
}
#[no_mangle]
pub unsafe extern "C" fn SingleLinkedList_new_int(mut i: libc::c_int) -> SingleLinkedList {
    let mut data: *mut libc::c_int = 0 as *mut libc::c_int;
    data = malloc(::std::mem::size_of::<libc::c_int>() as libc::c_ulong) as *mut libc::c_int;
    *data.offset(0 as libc::c_int as isize) = i;
    return SingleLinkedList_new(data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn SingleLinkedList_delete(
    mut head: SingleLinkedList,
    mut linklist_deallocator: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    let mut next: SingleLinkedList = 0 as *mut SingleLinkedList_struct;
    if head.is_null() {
        return;
    }
    loop {
        next = (*head).next;
        if !((*head).data).is_null() {
            linklist_deallocator.expect("non-null function pointer")((*head).data);
        }
        free(head as *mut libc::c_void);
        head = next;
        if head.is_null() {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn SingleLinkedList_prepend(
    mut l: SingleLinkedList,
    mut data: *mut libc::c_void,
) -> SingleLinkedList {
    let mut head: SingleLinkedList = SingleLinkedList_new(data);
    let ref mut fresh2 = (*head).next;
    *fresh2 = l;
    return head;
}
#[no_mangle]
pub unsafe extern "C" fn SingleLinkedList_prepend_int(
    mut l: SingleLinkedList,
    mut i: libc::c_int,
) -> SingleLinkedList {
    let mut data: *mut libc::c_int = 0 as *mut libc::c_int;
    data = malloc(::std::mem::size_of::<libc::c_int>() as libc::c_ulong) as *mut libc::c_int;
    *data.offset(0 as libc::c_int as isize) = i;
    return SingleLinkedList_prepend(l, data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn SingleLinkedList_get_data(mut l: SingleLinkedList) -> *mut libc::c_void {
    return (*l).data;
}
#[no_mangle]
pub unsafe extern "C" fn SingleLinkedList_get_next(mut l: SingleLinkedList) -> SingleLinkedList {
    return (*l).next;
}
#[no_mangle]
pub unsafe extern "C" fn SingleLinkedList_print(
    mut head: SingleLinkedList,
    mut linkedlist_print: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    if head.is_null() {
        return;
    }
    loop {
        if !((*head).data).is_null() {
            linkedlist_print.expect("non-null function pointer")((*head).data);
        }
        head = (*head).next;
        if head.is_null() {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn DoubleLinkedList_new(mut data: *mut libc::c_void) -> DoubleLinkedList {
    let mut head: DoubleLinkedList = 0 as *mut DoubleLinkedList_struct;
    head = gmalloc(::std::mem::size_of::<DoubleLinkedList_struct>() as libc::c_ulong)
        as *mut DoubleLinkedList_struct;
    let ref mut fresh3 = (*head).data;
    *fresh3 = data;
    let ref mut fresh4 = (*head).next;
    *fresh4 = 0 as DoubleLinkedList;
    let ref mut fresh5 = (*head).prev;
    *fresh5 = 0 as DoubleLinkedList;
    return head;
}
#[no_mangle]
pub unsafe extern "C" fn DoubleLinkedList_delete(
    mut head: DoubleLinkedList,
    mut linklist_deallocator: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    let mut next: DoubleLinkedList = 0 as *mut DoubleLinkedList_struct;
    if head.is_null() {
        return;
    }
    loop {
        next = (*head).next;
        if !((*head).data).is_null() {
            linklist_deallocator.expect("non-null function pointer")((*head).data);
        }
        free(head as *mut libc::c_void);
        head = next;
        if head.is_null() {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn DoubleLinkedList_prepend(
    mut l: DoubleLinkedList,
    mut data: *mut libc::c_void,
) -> DoubleLinkedList {
    let mut head: DoubleLinkedList = DoubleLinkedList_new(data);
    if !l.is_null() {
        let ref mut fresh6 = (*head).next;
        *fresh6 = l;
        let ref mut fresh7 = (*l).prev;
        *fresh7 = head;
    }
    return head;
}
#[no_mangle]
pub unsafe extern "C" fn DoubleLinkedList_get_data(mut l: DoubleLinkedList) -> *mut libc::c_void {
    return (*l).data;
}
#[no_mangle]
pub unsafe extern "C" fn DoubleLinkedList_get_next(mut l: DoubleLinkedList) -> DoubleLinkedList {
    return (*l).next;
}
#[no_mangle]
pub unsafe extern "C" fn DoubleLinkedList_print(
    mut head: DoubleLinkedList,
    mut linkedlist_print: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    if head.is_null() {
        return;
    }
    loop {
        if !((*head).data).is_null() {
            linkedlist_print.expect("non-null function pointer")((*head).data);
        }
        head = (*head).next;
        if head.is_null() {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn DoubleLinkedList_delete_element(
    mut l: DoubleLinkedList,
    mut linklist_deallocator: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut head: *mut DoubleLinkedList,
) {
    let mut next: DoubleLinkedList = 0 as *mut DoubleLinkedList_struct;
    let mut prev: DoubleLinkedList = 0 as *mut DoubleLinkedList_struct;
    if !l.is_null() {
        next = (*l).next;
        prev = (*l).prev;
        if !((*l).data).is_null() {
            linklist_deallocator.expect("non-null function pointer")((*l).data);
        }
        free(l as *mut libc::c_void);
        l = 0 as DoubleLinkedList;
        if !next.is_null() {
            let ref mut fresh8 = (*next).prev;
            *fresh8 = prev;
        }
        if !prev.is_null() {
            let ref mut fresh9 = (*prev).next;
            *fresh9 = next;
        }
        if prev.is_null() {
            *head = next;
        }
    }
}
