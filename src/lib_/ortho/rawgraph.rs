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
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn dtflatten(_: *mut Dt_t) -> *mut Dtlink_t;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn openIntSet() -> *mut Dt_t;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtlink_s {
    pub right: *mut Dtlink_t,
    pub hl: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _hash: libc::c_uint,
    pub _left: *mut Dtlink_t,
}
pub type Dtlink_t = _dtlink_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dthold_s {
    pub hdr: Dtlink_t,
    pub obj: *mut libc::c_void,
}
pub type Dthold_t = _dthold_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtdisc_s {
    pub key: libc::c_int,
    pub size: libc::c_int,
    pub link: libc::c_int,
    pub makef: Dtmake_f,
    pub freef: Dtfree_f,
    pub comparf: Dtcompar_f,
    pub hashf: Dthash_f,
    pub memoryf: Dtmemory_f,
    pub eventf: Dtevent_f,
}
pub type Dtevent_f = Option<
    unsafe extern "C" fn(*mut Dt_t, libc::c_int, *mut libc::c_void, *mut Dtdisc_t) -> libc::c_int,
>;
pub type Dtdisc_t = _dtdisc_s;
pub type Dt_t = _dt_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dt_s {
    pub searchf: Dtsearch_f,
    pub disc: *mut Dtdisc_t,
    pub data: *mut Dtdata_t,
    pub memoryf: Dtmemory_f,
    pub meth: *mut Dtmethod_t,
    pub type_0: libc::c_int,
    pub nview: libc::c_int,
    pub view: *mut Dt_t,
    pub walk: *mut Dt_t,
    pub user: *mut libc::c_void,
}
pub type Dtmethod_t = _dtmethod_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtmethod_s {
    pub searchf: Dtsearch_f,
    pub type_0: libc::c_int,
}
pub type Dtsearch_f =
    Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, libc::c_int) -> *mut libc::c_void>;
pub type Dtmemory_f = Option<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, size_t, *mut Dtdisc_t) -> *mut libc::c_void,
>;
pub type Dtdata_t = _dtdata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtdata_s {
    pub type_0: libc::c_int,
    pub here: *mut Dtlink_t,
    pub hh: C2RustUnnamed_0,
    pub ntab: libc::c_int,
    pub size: libc::c_int,
    pub loop_0: libc::c_int,
    pub minp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _htab: *mut *mut Dtlink_t,
    pub _head: *mut Dtlink_t,
}
pub type Dthash_f =
    Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> libc::c_uint>;
pub type Dtcompar_f = Option<
    unsafe extern "C" fn(
        *mut Dt_t,
        *mut libc::c_void,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> libc::c_int,
>;
pub type Dtfree_f = Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> ()>;
pub type Dtmake_f =
    Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> *mut libc::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vertex {
    pub color: libc::c_int,
    pub topsort_order: libc::c_int,
    pub adj_list: *mut Dt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rawgraph {
    pub nvs: libc::c_int,
    pub vertices: *mut vertex,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intitem {
    pub id: libc::c_int,
    pub link: Dtlink_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack {
    pub top: libc::c_int,
    pub vals: *mut libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn make_graph(mut n: libc::c_int) -> *mut rawgraph {
    let mut i: libc::c_int = 0;
    let mut g: *mut rawgraph =
        zmalloc(::std::mem::size_of::<rawgraph>() as libc::c_ulong) as *mut rawgraph;
    (*g).nvs = n;
    let ref mut fresh0 = (*g).vertices;
    *fresh0 = gcalloc(
        n as size_t,
        ::std::mem::size_of::<vertex>() as libc::c_ulong,
    ) as *mut vertex;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh1 = (*((*g).vertices).offset(i as isize)).adj_list;
        *fresh1 = openIntSet();
        (*((*g).vertices).offset(i as isize)).color = 0 as libc::c_int;
        i += 1;
    }
    return g;
}
#[no_mangle]
pub unsafe extern "C" fn free_graph(mut g: *mut rawgraph) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*g).nvs {
        dtclose((*((*g).vertices).offset(i as isize)).adj_list);
        i += 1;
    }
    free((*g).vertices as *mut libc::c_void);
    free(g as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn insert_edge(
    mut g: *mut rawgraph,
    mut v1: libc::c_int,
    mut v2: libc::c_int,
) {
    let mut obj: intitem = intitem {
        id: 0,
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
    };
    obj.id = v2;
    (Some(
        ((*(*((*g).vertices).offset(v1 as isize)).adj_list).searchf)
            .expect("non-null function pointer"),
    ))
    .expect("non-null function pointer")(
        (*((*g).vertices).offset(v1 as isize)).adj_list,
        &mut obj as *mut intitem as *mut libc::c_void,
        0o1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn remove_redge(
    mut g: *mut rawgraph,
    mut v1: libc::c_int,
    mut v2: libc::c_int,
) {
    let mut obj: intitem = intitem {
        id: 0,
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
    };
    obj.id = v2;
    (Some(
        ((*(*((*g).vertices).offset(v1 as isize)).adj_list).searchf)
            .expect("non-null function pointer"),
    ))
    .expect("non-null function pointer")(
        (*((*g).vertices).offset(v1 as isize)).adj_list,
        &mut obj as *mut intitem as *mut libc::c_void,
        0o2 as libc::c_int,
    );
    obj.id = v1;
    (Some(
        ((*(*((*g).vertices).offset(v2 as isize)).adj_list).searchf)
            .expect("non-null function pointer"),
    ))
    .expect("non-null function pointer")(
        (*((*g).vertices).offset(v2 as isize)).adj_list,
        &mut obj as *mut intitem as *mut libc::c_void,
        0o2 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn edge_exists(
    mut g: *mut rawgraph,
    mut v1: libc::c_int,
    mut v2: libc::c_int,
) -> bool {
    return !((Some(
        ((*(*((*g).vertices).offset(v1 as isize)).adj_list).searchf)
            .expect("non-null function pointer"),
    ))
    .expect("non-null function pointer")(
        (*((*g).vertices).offset(v1 as isize)).adj_list,
        &mut v2 as *mut libc::c_int as *mut libc::c_void,
        0o1000 as libc::c_int,
    ))
    .is_null();
}
unsafe extern "C" fn mkStack(mut i: libc::c_int) -> *mut stack {
    let mut sp: *mut stack = zmalloc(::std::mem::size_of::<stack>() as libc::c_ulong) as *mut stack;
    let ref mut fresh2 = (*sp).vals;
    *fresh2 = gcalloc(
        i as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    (*sp).top = -(1 as libc::c_int);
    return sp;
}
unsafe extern "C" fn freeStack(mut s: *mut stack) {
    free((*s).vals as *mut libc::c_void);
    free(s as *mut libc::c_void);
}
unsafe extern "C" fn pushStack(mut s: *mut stack, mut i: libc::c_int) {
    let ref mut fresh3 = (*s).top;
    *fresh3 += 1;
    *((*s).vals).offset((*s).top as isize) = i;
}
unsafe extern "C" fn popStack(mut s: *mut stack) -> libc::c_int {
    let mut v: libc::c_int = 0;
    if (*s).top == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    v = *((*s).vals).offset((*s).top as isize);
    let ref mut fresh4 = (*s).top;
    *fresh4 -= 1;
    return v;
}
unsafe extern "C" fn DFS_visit(
    mut g: *mut rawgraph,
    mut v: libc::c_int,
    mut time: libc::c_int,
    mut sp: *mut stack,
) -> libc::c_int {
    let mut adj: *mut Dt_t = 0 as *mut Dt_t;
    let mut link: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut id: libc::c_int = 0;
    let mut vp: *mut vertex = 0 as *mut vertex;
    vp = ((*g).vertices).offset(v as isize);
    (*vp).color = 1 as libc::c_int;
    adj = (*vp).adj_list;
    time = time + 1 as libc::c_int;
    link = dtflatten(adj);
    while !link.is_null() {
        id = (*((if (*(*adj).disc).link < 0 as libc::c_int {
            (*(link as *mut Dthold_t)).obj
        } else {
            (link as *mut libc::c_char).offset(-((*(*adj).disc).link as isize)) as *mut libc::c_void
        }) as *mut intitem))
            .id;
        if (*((*g).vertices).offset(id as isize)).color == 0 as libc::c_int {
            time = DFS_visit(g, id, time, sp);
        }
        link = (*link).right;
    }
    (*vp).color = 2 as libc::c_int;
    pushStack(sp, v);
    return time + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn top_sort(mut g: *mut rawgraph) {
    let mut i: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut time: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut sp: *mut stack = 0 as *mut stack;
    if (*g).nvs == 0 as libc::c_int {
        return;
    }
    if (*g).nvs == 1 as libc::c_int {
        (*((*g).vertices).offset(0 as libc::c_int as isize)).topsort_order = count;
        return;
    }
    sp = mkStack((*g).nvs);
    i = 0 as libc::c_int;
    while i < (*g).nvs {
        if (*((*g).vertices).offset(i as isize)).color == 0 as libc::c_int {
            time = DFS_visit(g, i, time, sp);
        }
        i += 1;
    }
    loop {
        v = popStack(sp);
        if !(v >= 0 as libc::c_int) {
            break;
        }
        (*((*g).vertices).offset(v as isize)).topsort_order = count;
        count += 1;
    }
    freeStack(sp);
}
