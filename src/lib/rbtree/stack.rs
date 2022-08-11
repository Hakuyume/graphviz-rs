#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type intptr_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stk_stack_node {
    pub info: *mut libc::c_void,
    pub next: *mut stk_stack_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stk_stack {
    pub top: *mut stk_stack_node,
    pub tail: *mut stk_stack_node,
}
#[no_mangle]
pub unsafe extern "C" fn StackNotEmpty(mut theStack: *mut stk_stack) -> intptr_t {
    return if !theStack.is_null() {
        (*theStack).top as intptr_t
    } else {
        0 as libc::c_int as libc::c_long
    };
}
#[no_mangle]
pub unsafe extern "C" fn StackJoin(
    mut stack1: *mut stk_stack,
    mut stack2: *mut stk_stack,
) -> *mut stk_stack {
    if ((*stack1).tail).is_null() {
        free(stack1 as *mut libc::c_void);
        return stack2;
    } else {
        let ref mut fresh0 = (*(*stack1).tail).next;
        *fresh0 = (*stack2).top;
        let ref mut fresh1 = (*stack1).tail;
        *fresh1 = (*stack2).tail;
        free(stack2 as *mut libc::c_void);
        return stack1;
    };
}
#[no_mangle]
pub unsafe extern "C" fn StackCreate() -> *mut stk_stack {
    let mut newStack: *mut stk_stack = 0 as *mut stk_stack;
    newStack = malloc(::std::mem::size_of::<stk_stack>() as libc::c_ulong)
        as *mut stk_stack;
    if newStack.is_null() {
        return 0 as *mut stk_stack;
    }
    let ref mut fresh2 = (*newStack).tail;
    *fresh2 = 0 as *mut stk_stack_node;
    let ref mut fresh3 = (*newStack).top;
    *fresh3 = *fresh2;
    return newStack;
}
#[no_mangle]
pub unsafe extern "C" fn StackPush(
    mut theStack: *mut stk_stack,
    mut newInfoPointer: *mut libc::c_void,
) -> libc::c_int {
    let mut newNode: *mut stk_stack_node = 0 as *mut stk_stack_node;
    if ((*theStack).top).is_null() {
        newNode = malloc(::std::mem::size_of::<stk_stack_node>() as libc::c_ulong)
            as *mut stk_stack_node;
        if newNode.is_null() {
            return -(1 as libc::c_int);
        }
        let ref mut fresh4 = (*newNode).info;
        *fresh4 = newInfoPointer;
        let ref mut fresh5 = (*newNode).next;
        *fresh5 = (*theStack).top;
        let ref mut fresh6 = (*theStack).top;
        *fresh6 = newNode;
        let ref mut fresh7 = (*theStack).tail;
        *fresh7 = newNode;
    } else {
        newNode = malloc(::std::mem::size_of::<stk_stack_node>() as libc::c_ulong)
            as *mut stk_stack_node;
        if newNode.is_null() {
            return -(1 as libc::c_int);
        }
        let ref mut fresh8 = (*newNode).info;
        *fresh8 = newInfoPointer;
        let ref mut fresh9 = (*newNode).next;
        *fresh9 = (*theStack).top;
        let ref mut fresh10 = (*theStack).top;
        *fresh10 = newNode;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn StackPop(mut theStack: *mut stk_stack) -> *mut libc::c_void {
    let mut popInfo: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut oldNode: *mut stk_stack_node = 0 as *mut stk_stack_node;
    if !((*theStack).top).is_null() {
        popInfo = (*(*theStack).top).info;
        oldNode = (*theStack).top;
        let ref mut fresh11 = (*theStack).top;
        *fresh11 = (*(*theStack).top).next;
        free(oldNode as *mut libc::c_void);
        if ((*theStack).top).is_null() {
            let ref mut fresh12 = (*theStack).tail;
            *fresh12 = 0 as *mut stk_stack_node;
        }
    } else {
        popInfo = 0 as *mut libc::c_void;
    }
    return popInfo;
}
#[no_mangle]
pub unsafe extern "C" fn StackDestroy(
    mut theStack: *mut stk_stack,
    mut DestFunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    if !theStack.is_null() {
        let mut x: *mut stk_stack_node = (*theStack).top;
        let mut y: *mut stk_stack_node = 0 as *mut stk_stack_node;
        while !x.is_null() {
            y = (*x).next;
            DestFunc.expect("non-null function pointer")((*x).info);
            free(x as *mut libc::c_void);
            x = y;
        }
        free(theStack as *mut libc::c_void);
    }
}
