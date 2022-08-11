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
    fn StackCreate() -> *mut stk_stack;
    fn StackPush(theStack: *mut stk_stack, newInfoPointer: *mut libc::c_void) -> libc::c_int;
    fn StackDestroy(
        theStack: *mut stk_stack,
        DestFunc: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn NullFunction(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rb_red_blk_node {
    pub key: *mut libc::c_void,
    pub info: *mut libc::c_void,
    pub red: libc::c_int,
    pub left: *mut rb_red_blk_node,
    pub right: *mut rb_red_blk_node,
    pub parent: *mut rb_red_blk_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rb_red_blk_tree {
    pub Compare:
        Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>,
    pub DestroyKey: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub DestroyInfo: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub PrintKey: Option<unsafe extern "C" fn(*const libc::c_void) -> ()>,
    pub PrintInfo: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub root: *mut rb_red_blk_node,
    pub nil: *mut rb_red_blk_node,
}
#[no_mangle]
pub unsafe extern "C" fn RBTreeCreate(
    mut CompFunc: Option<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
    mut DestFunc: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut InfoDestFunc: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut PrintFunc: Option<unsafe extern "C" fn(*const libc::c_void) -> ()>,
    mut PrintInfo: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) -> *mut rb_red_blk_tree {
    let mut newTree: *mut rb_red_blk_tree = 0 as *mut rb_red_blk_tree;
    let mut temp: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    newTree =
        malloc(::std::mem::size_of::<rb_red_blk_tree>() as libc::c_ulong) as *mut rb_red_blk_tree;
    if newTree.is_null() {
        return 0 as *mut rb_red_blk_tree;
    }
    let ref mut fresh0 = (*newTree).root;
    *fresh0 = 0 as *mut rb_red_blk_node;
    let ref mut fresh1 = (*newTree).nil;
    *fresh1 = *fresh0;
    let ref mut fresh2 = (*newTree).Compare;
    *fresh2 = CompFunc;
    let ref mut fresh3 = (*newTree).DestroyKey;
    *fresh3 = DestFunc;
    let ref mut fresh4 = (*newTree).PrintKey;
    *fresh4 = PrintFunc;
    let ref mut fresh5 = (*newTree).PrintInfo;
    *fresh5 = PrintInfo;
    let ref mut fresh6 = (*newTree).DestroyInfo;
    *fresh6 = InfoDestFunc;
    let ref mut fresh7 = (*newTree).nil;
    *fresh7 =
        malloc(::std::mem::size_of::<rb_red_blk_node>() as libc::c_ulong) as *mut rb_red_blk_node;
    temp = *fresh7;
    if temp.is_null() {
        free(newTree as *mut libc::c_void);
        return 0 as *mut rb_red_blk_tree;
    }
    let ref mut fresh8 = (*temp).right;
    *fresh8 = temp;
    let ref mut fresh9 = (*temp).left;
    *fresh9 = *fresh8;
    let ref mut fresh10 = (*temp).parent;
    *fresh10 = *fresh9;
    (*temp).red = 0 as libc::c_int;
    let ref mut fresh11 = (*temp).key;
    *fresh11 = 0 as *mut libc::c_void;
    let ref mut fresh12 = (*newTree).root;
    *fresh12 =
        malloc(::std::mem::size_of::<rb_red_blk_node>() as libc::c_ulong) as *mut rb_red_blk_node;
    temp = *fresh12;
    if temp.is_null() {
        free((*newTree).nil as *mut libc::c_void);
        free(newTree as *mut libc::c_void);
        return 0 as *mut rb_red_blk_tree;
    }
    let ref mut fresh13 = (*temp).right;
    *fresh13 = (*newTree).nil;
    let ref mut fresh14 = (*temp).left;
    *fresh14 = *fresh13;
    let ref mut fresh15 = (*temp).parent;
    *fresh15 = *fresh14;
    let ref mut fresh16 = (*temp).key;
    *fresh16 = 0 as *mut libc::c_void;
    (*temp).red = 0 as libc::c_int;
    return newTree;
}
unsafe extern "C" fn LeftRotate(mut tree: *mut rb_red_blk_tree, mut x: *mut rb_red_blk_node) {
    let mut y: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    let mut nil: *mut rb_red_blk_node = (*tree).nil;
    y = (*x).right;
    let ref mut fresh17 = (*x).right;
    *fresh17 = (*y).left;
    if (*y).left != nil {
        let ref mut fresh18 = (*(*y).left).parent;
        *fresh18 = x;
    }
    let ref mut fresh19 = (*y).parent;
    *fresh19 = (*x).parent;
    if x == (*(*x).parent).left {
        let ref mut fresh20 = (*(*x).parent).left;
        *fresh20 = y;
    } else {
        let ref mut fresh21 = (*(*x).parent).right;
        *fresh21 = y;
    }
    let ref mut fresh22 = (*y).left;
    *fresh22 = x;
    let ref mut fresh23 = (*x).parent;
    *fresh23 = y;
    if (*(*tree).nil).red == 0
        && !(b"nil not red in LeftRotate\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"!tree->nil->red && \"nil not red in LeftRotate\"\0" as *const u8
                as *const libc::c_char,
            b"red_black_tree.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                b"void LeftRotate(rb_red_blk_tree *, rb_red_blk_node *)\0",
            ))
            .as_ptr(),
        );
    };
}
unsafe extern "C" fn RightRotate(mut tree: *mut rb_red_blk_tree, mut y: *mut rb_red_blk_node) {
    let mut x: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    let mut nil: *mut rb_red_blk_node = (*tree).nil;
    x = (*y).left;
    let ref mut fresh24 = (*y).left;
    *fresh24 = (*x).right;
    if nil != (*x).right {
        let ref mut fresh25 = (*(*x).right).parent;
        *fresh25 = y;
    }
    let ref mut fresh26 = (*x).parent;
    *fresh26 = (*y).parent;
    if y == (*(*y).parent).left {
        let ref mut fresh27 = (*(*y).parent).left;
        *fresh27 = x;
    } else {
        let ref mut fresh28 = (*(*y).parent).right;
        *fresh28 = x;
    }
    let ref mut fresh29 = (*x).right;
    *fresh29 = y;
    let ref mut fresh30 = (*y).parent;
    *fresh30 = x;
    if (*(*tree).nil).red == 0
        && !(b"nil not red in RightRotate\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"!tree->nil->red && \"nil not red in RightRotate\"\0" as *const u8
                as *const libc::c_char,
            b"red_black_tree.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                b"void RightRotate(rb_red_blk_tree *, rb_red_blk_node *)\0",
            ))
            .as_ptr(),
        );
    };
}
unsafe extern "C" fn TreeInsertHelp(mut tree: *mut rb_red_blk_tree, mut z: *mut rb_red_blk_node) {
    let mut x: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    let mut y: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    let mut nil: *mut rb_red_blk_node = (*tree).nil;
    let ref mut fresh31 = (*z).right;
    *fresh31 = nil;
    let ref mut fresh32 = (*z).left;
    *fresh32 = *fresh31;
    y = (*tree).root;
    x = (*(*tree).root).left;
    while x != nil {
        y = x;
        if 1 as libc::c_int
            == ((*tree).Compare).expect("non-null function pointer")((*x).key, (*z).key)
        {
            x = (*x).left;
        } else {
            x = (*x).right;
        }
    }
    let ref mut fresh33 = (*z).parent;
    *fresh33 = y;
    if y == (*tree).root
        || 1 as libc::c_int
            == ((*tree).Compare).expect("non-null function pointer")((*y).key, (*z).key)
    {
        let ref mut fresh34 = (*y).left;
        *fresh34 = z;
    } else {
        let ref mut fresh35 = (*y).right;
        *fresh35 = z;
    }
    if (*(*tree).nil).red == 0
        && !(b"nil not red in TreeInsertHelp\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"!tree->nil->red && \"nil not red in TreeInsertHelp\"\0" as *const u8
                as *const libc::c_char,
            b"red_black_tree.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                b"void TreeInsertHelp(rb_red_blk_tree *, rb_red_blk_node *)\0",
            ))
            .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn RBTreeInsert(
    mut tree: *mut rb_red_blk_tree,
    mut key: *mut libc::c_void,
    mut info: *mut libc::c_void,
) -> *mut rb_red_blk_node {
    let mut y: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    let mut x: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    let mut newNode: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    x = malloc(::std::mem::size_of::<rb_red_blk_node>() as libc::c_ulong) as *mut rb_red_blk_node;
    if x.is_null() {
        return 0 as *mut rb_red_blk_node;
    }
    let ref mut fresh36 = (*x).key;
    *fresh36 = key;
    let ref mut fresh37 = (*x).info;
    *fresh37 = info;
    TreeInsertHelp(tree, x);
    newNode = x;
    (*x).red = 1 as libc::c_int;
    while (*(*x).parent).red != 0 {
        if (*x).parent == (*(*(*x).parent).parent).left {
            y = (*(*(*x).parent).parent).right;
            if (*y).red != 0 {
                (*(*x).parent).red = 0 as libc::c_int;
                (*y).red = 0 as libc::c_int;
                (*(*(*x).parent).parent).red = 1 as libc::c_int;
                x = (*(*x).parent).parent;
            } else {
                if x == (*(*x).parent).right {
                    x = (*x).parent;
                    LeftRotate(tree, x);
                }
                (*(*x).parent).red = 0 as libc::c_int;
                (*(*(*x).parent).parent).red = 1 as libc::c_int;
                RightRotate(tree, (*(*x).parent).parent);
            }
        } else {
            y = (*(*(*x).parent).parent).left;
            if (*y).red != 0 {
                (*(*x).parent).red = 0 as libc::c_int;
                (*y).red = 0 as libc::c_int;
                (*(*(*x).parent).parent).red = 1 as libc::c_int;
                x = (*(*x).parent).parent;
            } else {
                if x == (*(*x).parent).left {
                    x = (*x).parent;
                    RightRotate(tree, x);
                }
                (*(*x).parent).red = 0 as libc::c_int;
                (*(*(*x).parent).parent).red = 1 as libc::c_int;
                LeftRotate(tree, (*(*x).parent).parent);
            }
        }
    }
    (*(*(*tree).root).left).red = 0 as libc::c_int;
    return newNode;
}
#[no_mangle]
pub unsafe extern "C" fn TreeSuccessor(
    mut tree: *mut rb_red_blk_tree,
    mut x: *mut rb_red_blk_node,
) -> *mut rb_red_blk_node {
    let mut y: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    let mut nil: *mut rb_red_blk_node = (*tree).nil;
    let mut root: *mut rb_red_blk_node = (*tree).root;
    y = (*x).right;
    if nil != y {
        while (*y).left != nil {
            y = (*y).left;
        }
        return y;
    } else {
        y = (*x).parent;
        while x == (*y).right {
            x = y;
            y = (*y).parent;
        }
        if y == root {
            return nil;
        }
        return y;
    };
}
#[no_mangle]
pub unsafe extern "C" fn TreePredecessor(
    mut tree: *mut rb_red_blk_tree,
    mut x: *mut rb_red_blk_node,
) -> *mut rb_red_blk_node {
    let mut y: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    let mut nil: *mut rb_red_blk_node = (*tree).nil;
    let mut root: *mut rb_red_blk_node = (*tree).root;
    y = (*x).left;
    if nil != y {
        while (*y).right != nil {
            y = (*y).right;
        }
        return y;
    } else {
        y = (*x).parent;
        while x == (*y).left {
            if y == root {
                return nil;
            }
            x = y;
            y = (*y).parent;
        }
        return y;
    };
}
unsafe extern "C" fn InorderTreePrint(mut tree: *mut rb_red_blk_tree, mut x: *mut rb_red_blk_node) {
    let mut nil: *mut rb_red_blk_node = (*tree).nil;
    let mut root: *mut rb_red_blk_node = (*tree).root;
    if x != (*tree).nil {
        InorderTreePrint(tree, (*x).left);
        printf(b"info=\0" as *const u8 as *const libc::c_char);
        ((*tree).PrintInfo).expect("non-null function pointer")((*x).info);
        printf(b"  key=\0" as *const u8 as *const libc::c_char);
        ((*tree).PrintKey).expect("non-null function pointer")((*x).key);
        printf(b"  l->key=\0" as *const u8 as *const libc::c_char);
        if (*x).left == nil {
            printf(b"NULL\0" as *const u8 as *const libc::c_char);
        } else {
            ((*tree).PrintKey).expect("non-null function pointer")((*(*x).left).key);
        }
        printf(b"  r->key=\0" as *const u8 as *const libc::c_char);
        if (*x).right == nil {
            printf(b"NULL\0" as *const u8 as *const libc::c_char);
        } else {
            ((*tree).PrintKey).expect("non-null function pointer")((*(*x).right).key);
        }
        printf(b"  p->key=\0" as *const u8 as *const libc::c_char);
        if (*x).parent == root {
            printf(b"NULL\0" as *const u8 as *const libc::c_char);
        } else {
            ((*tree).PrintKey).expect("non-null function pointer")((*(*x).parent).key);
        }
        printf(
            b"  red=%i\n\0" as *const u8 as *const libc::c_char,
            (*x).red,
        );
        InorderTreePrint(tree, (*x).right);
    }
}
unsafe extern "C" fn TreeDestHelper(mut tree: *mut rb_red_blk_tree, mut x: *mut rb_red_blk_node) {
    let mut nil: *mut rb_red_blk_node = (*tree).nil;
    if x != nil {
        TreeDestHelper(tree, (*x).left);
        TreeDestHelper(tree, (*x).right);
        ((*tree).DestroyKey).expect("non-null function pointer")((*x).key);
        ((*tree).DestroyInfo).expect("non-null function pointer")((*x).info);
        free(x as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn RBTreeDestroy(mut tree: *mut rb_red_blk_tree) {
    TreeDestHelper(tree, (*(*tree).root).left);
    free((*tree).root as *mut libc::c_void);
    free((*tree).nil as *mut libc::c_void);
    free(tree as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn RBTreePrint(mut tree: *mut rb_red_blk_tree) {
    InorderTreePrint(tree, (*(*tree).root).left);
}
#[no_mangle]
pub unsafe extern "C" fn RBExactQuery(
    mut tree: *mut rb_red_blk_tree,
    mut q: *mut libc::c_void,
) -> *mut rb_red_blk_node {
    let mut x: *mut rb_red_blk_node = (*(*tree).root).left;
    let mut nil: *mut rb_red_blk_node = (*tree).nil;
    let mut compVal: libc::c_int = 0;
    if x == nil {
        return 0 as *mut rb_red_blk_node;
    }
    compVal = ((*tree).Compare).expect("non-null function pointer")(
        (*x).key,
        q as *mut libc::c_int as *const libc::c_void,
    );
    while 0 as libc::c_int != compVal {
        if 1 as libc::c_int == compVal {
            x = (*x).left;
        } else {
            x = (*x).right;
        }
        if x == nil {
            return 0 as *mut rb_red_blk_node;
        }
        compVal = ((*tree).Compare).expect("non-null function pointer")(
            (*x).key,
            q as *mut libc::c_int as *const libc::c_void,
        );
    }
    return x;
}
unsafe extern "C" fn RBDeleteFixUp(mut tree: *mut rb_red_blk_tree, mut x: *mut rb_red_blk_node) {
    let mut root: *mut rb_red_blk_node = (*(*tree).root).left;
    let mut w: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    while (*x).red == 0 && root != x {
        if x == (*(*x).parent).left {
            w = (*(*x).parent).right;
            if (*w).red != 0 {
                (*w).red = 0 as libc::c_int;
                (*(*x).parent).red = 1 as libc::c_int;
                LeftRotate(tree, (*x).parent);
                w = (*(*x).parent).right;
            }
            if (*(*w).right).red == 0 && (*(*w).left).red == 0 {
                (*w).red = 1 as libc::c_int;
                x = (*x).parent;
            } else {
                if (*(*w).right).red == 0 {
                    (*(*w).left).red = 0 as libc::c_int;
                    (*w).red = 1 as libc::c_int;
                    RightRotate(tree, w);
                    w = (*(*x).parent).right;
                }
                (*w).red = (*(*x).parent).red;
                (*(*x).parent).red = 0 as libc::c_int;
                (*(*w).right).red = 0 as libc::c_int;
                LeftRotate(tree, (*x).parent);
                x = root;
            }
        } else {
            w = (*(*x).parent).left;
            if (*w).red != 0 {
                (*w).red = 0 as libc::c_int;
                (*(*x).parent).red = 1 as libc::c_int;
                RightRotate(tree, (*x).parent);
                w = (*(*x).parent).left;
            }
            if (*(*w).right).red == 0 && (*(*w).left).red == 0 {
                (*w).red = 1 as libc::c_int;
                x = (*x).parent;
            } else {
                if (*(*w).left).red == 0 {
                    (*(*w).right).red = 0 as libc::c_int;
                    (*w).red = 1 as libc::c_int;
                    LeftRotate(tree, w);
                    w = (*(*x).parent).left;
                }
                (*w).red = (*(*x).parent).red;
                (*(*x).parent).red = 0 as libc::c_int;
                (*(*w).left).red = 0 as libc::c_int;
                RightRotate(tree, (*x).parent);
                x = root;
            }
        }
    }
    (*x).red = 0 as libc::c_int;
    if (*(*tree).nil).red == 0
        && !(b"nil not black in RBDeleteFixUp\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"!tree->nil->red && \"nil not black in RBDeleteFixUp\"\0" as *const u8
                as *const libc::c_char,
            b"red_black_tree.c\0" as *const u8 as *const libc::c_char,
            573 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"void RBDeleteFixUp(rb_red_blk_tree *, rb_red_blk_node *)\0",
            ))
            .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn RBDelete(mut tree: *mut rb_red_blk_tree, mut z: *mut rb_red_blk_node) {
    let mut y: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    let mut x: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    let mut nil: *mut rb_red_blk_node = (*tree).nil;
    let mut root: *mut rb_red_blk_node = (*tree).root;
    y = if (*z).left == nil || (*z).right == nil {
        z
    } else {
        TreeSuccessor(tree, z)
    };
    x = if (*y).left == nil {
        (*y).right
    } else {
        (*y).left
    };
    let ref mut fresh38 = (*x).parent;
    *fresh38 = (*y).parent;
    if root == *fresh38 {
        let ref mut fresh39 = (*root).left;
        *fresh39 = x;
    } else if y == (*(*y).parent).left {
        let ref mut fresh40 = (*(*y).parent).left;
        *fresh40 = x;
    } else {
        let ref mut fresh41 = (*(*y).parent).right;
        *fresh41 = x;
    }
    if y != z {
        if y != (*tree).nil
            && !(b"y is nil in RBDelete\0" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"y!=tree->nil && \"y is nil in RBDelete\"\0" as *const u8 as *const libc::c_char,
                b"red_black_tree.c\0" as *const u8 as *const libc::c_char,
                612 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                    b"void RBDelete(rb_red_blk_tree *, rb_red_blk_node *)\0",
                ))
                .as_ptr(),
            );
        }
        if (*y).red == 0 {
            RBDeleteFixUp(tree, x);
        }
        ((*tree).DestroyKey).expect("non-null function pointer")((*z).key);
        ((*tree).DestroyInfo).expect("non-null function pointer")((*z).info);
        let ref mut fresh42 = (*y).left;
        *fresh42 = (*z).left;
        let ref mut fresh43 = (*y).right;
        *fresh43 = (*z).right;
        let ref mut fresh44 = (*y).parent;
        *fresh44 = (*z).parent;
        (*y).red = (*z).red;
        let ref mut fresh45 = (*(*z).right).parent;
        *fresh45 = y;
        let ref mut fresh46 = (*(*z).left).parent;
        *fresh46 = *fresh45;
        if z == (*(*z).parent).left {
            let ref mut fresh47 = (*(*z).parent).left;
            *fresh47 = y;
        } else {
            let ref mut fresh48 = (*(*z).parent).right;
            *fresh48 = y;
        }
        free(z as *mut libc::c_void);
    } else {
        ((*tree).DestroyKey).expect("non-null function pointer")((*y).key);
        ((*tree).DestroyInfo).expect("non-null function pointer")((*y).info);
        if (*y).red == 0 {
            RBDeleteFixUp(tree, x);
        }
        free(y as *mut libc::c_void);
    }
    if (*(*tree).nil).red == 0
        && !(b"nil not black in RBDelete\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"!tree->nil->red && \"nil not black in RBDelete\"\0" as *const u8
                as *const libc::c_char,
            b"red_black_tree.c\0" as *const u8 as *const libc::c_char,
            637 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                b"void RBDelete(rb_red_blk_tree *, rb_red_blk_node *)\0",
            ))
            .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn RBEnumerate(
    mut tree: *mut rb_red_blk_tree,
    mut low: *mut libc::c_void,
    mut high: *mut libc::c_void,
) -> *mut stk_stack {
    let mut enumResultStack: *mut stk_stack = 0 as *mut stk_stack;
    let mut nil: *mut rb_red_blk_node = (*tree).nil;
    let mut x: *mut rb_red_blk_node = (*(*tree).root).left;
    let mut lastBest: *mut rb_red_blk_node = nil;
    enumResultStack = StackCreate();
    if enumResultStack.is_null() {
        return 0 as *mut stk_stack;
    }
    while nil != x {
        if 1 as libc::c_int == ((*tree).Compare).expect("non-null function pointer")((*x).key, high)
        {
            x = (*x).left;
        } else {
            lastBest = x;
            x = (*x).right;
        }
    }
    while lastBest != nil
        && 1 as libc::c_int
            != ((*tree).Compare).expect("non-null function pointer")(low, (*lastBest).key)
    {
        if StackPush(enumResultStack, lastBest as *mut libc::c_void) != 0 as libc::c_int {
            StackDestroy(
                enumResultStack,
                Some(NullFunction as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            );
            return 0 as *mut stk_stack;
        }
        lastBest = TreePredecessor(tree, lastBest);
    }
    return enumResultStack;
}
