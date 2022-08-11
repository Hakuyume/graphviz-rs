#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type cell;
    fn free(_: *mut libc::c_void);
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn PQinit();
    fn PQ_insert(np: *mut snode) -> libc::c_int;
    fn PQremove() -> *mut snode;
    fn PQupdate(n: *mut snode, d: libc::c_int);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snode {
    pub n_val: libc::c_int,
    pub n_idx: libc::c_int,
    pub n_dad: *mut snode,
    pub n_edge: *mut sedge,
    pub n_adj: libc::c_short,
    pub save_n_adj: libc::c_short,
    pub cells: [*mut cell; 2],
    pub adj_edge_list: *mut libc::c_int,
    pub index: libc::c_int,
    pub isVert: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sedge {
    pub weight: libc::c_double,
    pub cnt: libc::c_int,
    pub v1: libc::c_int,
    pub v2: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sgraph {
    pub nnodes: libc::c_int,
    pub nedges: libc::c_int,
    pub save_nnodes: libc::c_int,
    pub save_nedges: libc::c_int,
    pub nodes: *mut snode,
    pub edges: *mut sedge,
}
#[no_mangle]
pub unsafe extern "C" fn gsave(mut G: *mut sgraph) {
    let mut i: libc::c_int = 0;
    (*G).save_nnodes = (*G).nnodes;
    (*G).save_nedges = (*G).nedges;
    i = 0 as libc::c_int;
    while i < (*G).nnodes {
        (*((*G).nodes).offset(i as isize))
            .save_n_adj = (*((*G).nodes).offset(i as isize)).n_adj;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn reset(mut G: *mut sgraph) {
    let mut i: libc::c_int = 0;
    (*G).nnodes = (*G).save_nnodes;
    (*G).nedges = (*G).save_nedges;
    i = 0 as libc::c_int;
    while i < (*G).nnodes {
        (*((*G).nodes).offset(i as isize))
            .n_adj = (*((*G).nodes).offset(i as isize)).save_n_adj;
        i += 1;
    }
    while i < (*G).nnodes + 2 as libc::c_int {
        (*((*G).nodes).offset(i as isize)).n_adj = 0 as libc::c_int as libc::c_short;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn initSEdges(mut g: *mut sgraph, mut maxdeg: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut adj: *mut libc::c_int = gcalloc(
        (6 as libc::c_int * (*g).nnodes + 2 as libc::c_int * maxdeg) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let ref mut fresh0 = (*g).edges;
    *fresh0 = gcalloc(
        (3 as libc::c_int * (*g).nnodes + maxdeg) as size_t,
        ::std::mem::size_of::<sedge>() as libc::c_ulong,
    ) as *mut sedge;
    i = 0 as libc::c_int;
    while i < (*g).nnodes {
        let ref mut fresh1 = (*((*g).nodes).offset(i as isize)).adj_edge_list;
        *fresh1 = adj;
        adj = adj.offset(6 as libc::c_int as isize);
        i += 1;
    }
    while i < (*g).nnodes + 2 as libc::c_int {
        let ref mut fresh2 = (*((*g).nodes).offset(i as isize)).adj_edge_list;
        *fresh2 = adj;
        adj = adj.offset(maxdeg as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn createSGraph(mut nnodes: libc::c_int) -> *mut sgraph {
    let mut g: *mut sgraph = zmalloc(::std::mem::size_of::<sgraph>() as libc::c_ulong)
        as *mut sgraph;
    (*g).nnodes = 0 as libc::c_int;
    let ref mut fresh3 = (*g).nodes;
    *fresh3 = gcalloc(nnodes as size_t, ::std::mem::size_of::<snode>() as libc::c_ulong)
        as *mut snode;
    return g;
}
#[no_mangle]
pub unsafe extern "C" fn createSNode(mut g: *mut sgraph) -> *mut snode {
    let mut np: *mut snode = ((*g).nodes).offset((*g).nnodes as isize);
    (*np).index = (*g).nnodes;
    let ref mut fresh4 = (*g).nnodes;
    *fresh4 += 1;
    return np;
}
unsafe extern "C" fn addEdgeToNode(mut np: *mut snode, mut idx: libc::c_int) {
    *((*np).adj_edge_list).offset((*np).n_adj as isize) = idx;
    let ref mut fresh5 = (*np).n_adj;
    *fresh5 += 1;
}
#[no_mangle]
pub unsafe extern "C" fn createSEdge(
    mut g: *mut sgraph,
    mut v1: *mut snode,
    mut v2: *mut snode,
    mut wt: libc::c_double,
) -> *mut sedge {
    let mut e: *mut sedge = 0 as *mut sedge;
    let ref mut fresh6 = (*g).nedges;
    let fresh7 = *fresh6;
    *fresh6 = *fresh6 + 1;
    let mut idx: libc::c_int = fresh7;
    e = ((*g).edges).offset(idx as isize);
    (*e).v1 = (*v1).index;
    (*e).v2 = (*v2).index;
    (*e).weight = wt;
    (*e).cnt = 0 as libc::c_int;
    addEdgeToNode(v1, idx);
    addEdgeToNode(v2, idx);
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn freeSGraph(mut g: *mut sgraph) {
    free(
        (*((*g).nodes).offset(0 as libc::c_int as isize)).adj_edge_list
            as *mut libc::c_void,
    );
    free((*g).nodes as *mut libc::c_void);
    free((*g).edges as *mut libc::c_void);
    free(g as *mut libc::c_void);
}
unsafe extern "C" fn adjacentNode(
    mut g: *mut sgraph,
    mut e: *mut sedge,
    mut n: *mut snode,
) -> *mut snode {
    if (*e).v1 == (*n).index {
        return &mut *((*g).nodes).offset((*e).v2 as isize) as *mut snode
    } else {
        return &mut *((*g).nodes).offset((*e).v1 as isize) as *mut snode
    };
}
#[no_mangle]
pub unsafe extern "C" fn shortPath(
    mut g: *mut sgraph,
    mut from: *mut snode,
    mut to: *mut snode,
) -> libc::c_int {
    let mut n: *mut snode = 0 as *mut snode;
    let mut e: *mut sedge = 0 as *mut sedge;
    let mut adjn: *mut snode = 0 as *mut snode;
    let mut d: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < (*g).nnodes {
        let mut temp: *mut snode = &mut *((*g).nodes).offset(x as isize) as *mut snode;
        (*temp).n_val = -(2147483647 as libc::c_int) - 1 as libc::c_int;
        x += 1;
    }
    PQinit();
    if PQ_insert(from) != 0 {
        return 1 as libc::c_int;
    }
    let ref mut fresh8 = (*from).n_dad;
    *fresh8 = 0 as *mut snode;
    (*from).n_val = 0 as libc::c_int;
    loop {
        n = PQremove();
        if n.is_null() {
            break;
        }
        (*n).n_val *= -(1 as libc::c_int);
        if n == to {
            break;
        }
        y = 0 as libc::c_int;
        while y < (*n).n_adj as libc::c_int {
            e = &mut *((*g).edges)
                .offset(*((*n).adj_edge_list).offset(y as isize) as isize) as *mut sedge;
            adjn = adjacentNode(g, e, n);
            if (*adjn).n_val < 0 as libc::c_int {
                d = -((*n).n_val as libc::c_double + (*e).weight) as libc::c_int;
                if (*adjn).n_val == -(2147483647 as libc::c_int) - 1 as libc::c_int {
                    (*adjn).n_val = d;
                    if PQ_insert(adjn) != 0 {
                        return 1 as libc::c_int;
                    }
                    let ref mut fresh9 = (*adjn).n_dad;
                    *fresh9 = n;
                    let ref mut fresh10 = (*adjn).n_edge;
                    *fresh10 = e;
                } else if (*adjn).n_val < d {
                    PQupdate(adjn, d);
                    let ref mut fresh11 = (*adjn).n_dad;
                    *fresh11 = n;
                    let ref mut fresh12 = (*adjn).n_edge;
                    *fresh12 = e;
                }
            }
            y += 1;
        }
    }
    return 0 as libc::c_int;
}
