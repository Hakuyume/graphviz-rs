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
    fn rand() -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn mkQueue(_: *mut Queue, _: libc::c_int);
    fn freeQueue(_: *mut Queue);
    fn bfs(_: libc::c_int, _: *mut vtx_data, _: libc::c_int, _: *mut DistType, _: *mut Queue);
    fn dijkstra(_: libc::c_int, _: *mut vtx_data, _: libc::c_int, _: *mut DistType);
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
pub type qsort_cmpf =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vtx_data {
    pub nedges: libc::c_int,
    pub edges: *mut libc::c_int,
    pub ewgts: *mut libc::c_float,
    pub eweights: *mut libc::c_float,
    pub edists: *mut libc::c_float,
}
pub type DistType = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Queue {
    pub data: *mut libc::c_int,
    pub queueSize: libc::c_int,
    pub end: libc::c_int,
    pub start: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn common_neighbors(
    mut graph: *mut vtx_data,
    mut v: libc::c_int,
    mut u: libc::c_int,
    mut v_vector: *mut libc::c_int,
) -> libc::c_int {
    let mut neighbor: libc::c_int = 0;
    let mut num_shared_neighbors: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    j = 1 as libc::c_int;
    while j < (*graph.offset(u as isize)).nedges {
        neighbor = *((*graph.offset(u as isize)).edges).offset(j as isize);
        if *v_vector.offset(neighbor as isize) > 0 as libc::c_int {
            num_shared_neighbors += 1;
        }
        j += 1;
    }
    return num_shared_neighbors;
}
#[no_mangle]
pub unsafe extern "C" fn fill_neighbors_vec_unweighted(
    mut graph: *mut vtx_data,
    mut vtx: libc::c_int,
    mut vtx_vec: *mut libc::c_int,
) {
    let mut j: libc::c_int = 0;
    j = 1 as libc::c_int;
    while j < (*graph.offset(vtx as isize)).nedges {
        *vtx_vec.offset(*((*graph.offset(vtx as isize)).edges).offset(j as isize) as isize) =
            1 as libc::c_int;
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn empty_neighbors_vec(
    mut graph: *mut vtx_data,
    mut vtx: libc::c_int,
    mut vtx_vec: *mut libc::c_int,
) {
    let mut j: libc::c_int = 0;
    j = 1 as libc::c_int;
    while j < (*graph.offset(vtx as isize)).nedges {
        *vtx_vec.offset(*((*graph.offset(vtx as isize)).edges).offset(j as isize) as isize) =
            0 as libc::c_int;
        j += 1;
    }
}
unsafe extern "C" fn compute_apsp_dijkstra(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
) -> *mut *mut DistType {
    let mut i: libc::c_int = 0;
    let mut storage: *mut DistType = 0 as *mut DistType;
    let mut dij: *mut *mut DistType = 0 as *mut *mut DistType;
    storage = gcalloc(
        (n * n) as size_t,
        ::std::mem::size_of::<DistType>() as libc::c_ulong,
    ) as *mut DistType;
    dij = gcalloc(
        n as size_t,
        ::std::mem::size_of::<*mut DistType>() as libc::c_ulong,
    ) as *mut *mut DistType;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh0 = *dij.offset(i as isize);
        *fresh0 = storage.offset((i * n) as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        dijkstra(i, graph, n, *dij.offset(i as isize));
        i += 1;
    }
    return dij;
}
unsafe extern "C" fn compute_apsp_simple(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
) -> *mut *mut DistType {
    let mut i: libc::c_int = 0;
    let mut storage: *mut DistType = gcalloc(
        (n * n) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut dij: *mut *mut DistType = 0 as *mut *mut DistType;
    let mut Q: Queue = Queue {
        data: 0 as *mut libc::c_int,
        queueSize: 0,
        end: 0,
        start: 0,
    };
    dij = gcalloc(
        n as size_t,
        ::std::mem::size_of::<*mut DistType>() as libc::c_ulong,
    ) as *mut *mut DistType;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh1 = *dij.offset(i as isize);
        *fresh1 = storage.offset((i * n) as isize);
        i += 1;
    }
    mkQueue(&mut Q, n);
    i = 0 as libc::c_int;
    while i < n {
        bfs(i, graph, n, *dij.offset(i as isize), &mut Q);
        i += 1;
    }
    freeQueue(&mut Q);
    return dij;
}
#[no_mangle]
pub unsafe extern "C" fn compute_apsp(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
) -> *mut *mut DistType {
    if !((*graph).ewgts).is_null() {
        return compute_apsp_dijkstra(graph, n);
    } else {
        return compute_apsp_simple(graph, n);
    };
}
#[no_mangle]
pub unsafe extern "C" fn compute_apsp_artifical_weights(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
) -> *mut *mut DistType {
    let mut Dij: *mut *mut DistType = 0 as *mut *mut DistType;
    let mut old_weights: *mut libc::c_float = (*graph.offset(0 as libc::c_int as isize)).ewgts;
    compute_new_weights(graph, n);
    Dij = compute_apsp_dijkstra(graph, n);
    restore_old_weights(graph, n, old_weights);
    return Dij;
}
unsafe extern "C" fn split_by_place(
    mut place: *mut libc::c_double,
    mut nodes: *mut libc::c_int,
    mut first: libc::c_int,
    mut last: libc::c_int,
    mut middle: *mut libc::c_int,
) {
    let mut splitter: libc::c_uint = (rand() as libc::c_uint
        | (rand() as libc::c_uint) << 16 as libc::c_int)
        .wrapping_rem((last - first + 1 as libc::c_int) as libc::c_uint)
        .wrapping_add(first as libc::c_uint);
    let mut val: libc::c_int = 0;
    let mut place_val: libc::c_double = 0.;
    let mut left: libc::c_int = first + 1 as libc::c_int;
    let mut right: libc::c_int = last;
    let mut temp: libc::c_int = 0;
    val = *nodes.offset(splitter as isize);
    *nodes.offset(splitter as isize) = *nodes.offset(first as isize);
    *nodes.offset(first as isize) = val;
    place_val = *place.offset(val as isize);
    while left < right {
        while left < right && *place.offset(*nodes.offset(left as isize) as isize) <= place_val {
            left += 1;
        }
        while left < right && *place.offset(*nodes.offset(right as isize) as isize) > place_val {
            right -= 1;
        }
        if left < right {
            temp = *nodes.offset(left as isize);
            *nodes.offset(left as isize) = *nodes.offset(right as isize);
            *nodes.offset(right as isize) = temp;
            left += 1;
            right -= 1;
        }
    }
    if *place.offset(*nodes.offset(left as isize) as isize) > place_val {
        left -= 1;
    }
    *middle = left;
    *nodes.offset(first as isize) = *nodes.offset(left as isize);
    *nodes.offset(left as isize) = val;
}
#[no_mangle]
pub unsafe extern "C" fn distance_kD(
    mut coords: *mut *mut libc::c_double,
    mut dim: libc::c_int,
    mut i: libc::c_int,
    mut j: libc::c_int,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < dim {
        sum += (*(*coords.offset(k as isize)).offset(i as isize)
            - *(*coords.offset(k as isize)).offset(j as isize))
            * (*(*coords.offset(k as isize)).offset(i as isize)
                - *(*coords.offset(k as isize)).offset(j as isize));
        k += 1;
    }
    return sqrt(sum);
}
static mut fvals: *mut libc::c_float = 0 as *const libc::c_float as *mut libc::c_float;
unsafe extern "C" fn fcmpf(mut ip1: *mut libc::c_int, mut ip2: *mut libc::c_int) -> libc::c_int {
    let mut d1: libc::c_float = *fvals.offset(*ip1 as isize);
    let mut d2: libc::c_float = *fvals.offset(*ip2 as isize);
    if d1 < d2 {
        return -(1 as libc::c_int);
    } else if d1 > d2 {
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn quicksort_placef(
    mut place: *mut libc::c_float,
    mut ordering: *mut libc::c_int,
    mut first: libc::c_int,
    mut last: libc::c_int,
) {
    if first < last {
        fvals = place;
        qsort(
            ordering.offset(first as isize) as *mut libc::c_void,
            (last - first + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option<unsafe extern "C" fn(*mut libc::c_int, *mut libc::c_int) -> libc::c_int>,
                qsort_cmpf,
            >(Some(
                fcmpf as unsafe extern "C" fn(*mut libc::c_int, *mut libc::c_int) -> libc::c_int,
            )),
        );
    }
}
unsafe extern "C" fn sorted_place(
    mut place: *mut libc::c_double,
    mut ordering: *mut libc::c_int,
    mut first: libc::c_int,
    mut last: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut isSorted: libc::c_int = 1 as libc::c_int;
    i = first + 1 as libc::c_int;
    while i <= last && isSorted != 0 {
        if *place.offset(*ordering.offset((i - 1 as libc::c_int) as isize) as isize)
            > *place.offset(*ordering.offset(i as isize) as isize)
        {
            isSorted = 0 as libc::c_int;
        }
        i += 1;
    }
    return isSorted;
}
#[no_mangle]
pub unsafe extern "C" fn quicksort_place(
    mut place: *mut libc::c_double,
    mut ordering: *mut libc::c_int,
    mut first: libc::c_int,
    mut last: libc::c_int,
) {
    if first < last {
        let mut middle: libc::c_int = 0;
        split_by_place(place, ordering, first, last, &mut middle);
        quicksort_place(place, ordering, first, middle - 1 as libc::c_int);
        quicksort_place(place, ordering, middle + 1 as libc::c_int, last);
        if sorted_place(place, ordering, first, middle - 1 as libc::c_int) == 0 {
            quicksort_place(place, ordering, first, middle - 1 as libc::c_int);
        }
        if sorted_place(place, ordering, middle + 1 as libc::c_int, last) == 0 {
            quicksort_place(place, ordering, middle + 1 as libc::c_int, last);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn compute_new_weights(mut graph: *mut vtx_data, mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nedges: libc::c_int = 0 as libc::c_int;
    let mut weights: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut vtx_vec: *mut libc::c_int = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut deg_i: libc::c_int = 0;
    let mut deg_j: libc::c_int = 0;
    let mut neighbor: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        nedges += (*graph.offset(i as isize)).nedges;
        i += 1;
    }
    weights = gcalloc(
        nedges as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    i = 0 as libc::c_int;
    while i < n {
        *vtx_vec.offset(i as isize) = 0 as libc::c_int;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh2 = (*graph.offset(i as isize)).ewgts;
        *fresh2 = weights;
        fill_neighbors_vec_unweighted(graph, i, vtx_vec);
        deg_i = (*graph.offset(i as isize)).nedges - 1 as libc::c_int;
        j = 1 as libc::c_int;
        while j <= deg_i {
            neighbor = *((*graph.offset(i as isize)).edges).offset(j as isize);
            deg_j = (*graph.offset(neighbor as isize)).nedges - 1 as libc::c_int;
            *weights.offset(j as isize) = (deg_i + deg_j
                - 2 as libc::c_int * common_neighbors(graph, i, neighbor, vtx_vec))
                as libc::c_float;
            j += 1;
        }
        empty_neighbors_vec(graph, i, vtx_vec);
        weights = weights.offset((*graph.offset(i as isize)).nedges as isize);
        i += 1;
    }
    free(vtx_vec as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn restore_old_weights(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut old_weights: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    free((*graph.offset(0 as libc::c_int as isize)).ewgts as *mut libc::c_void);
    let ref mut fresh3 = (*graph.offset(0 as libc::c_int as isize)).ewgts;
    *fresh3 = 0 as *mut libc::c_float;
    if !old_weights.is_null() {
        i = 0 as libc::c_int;
        while i < n {
            let ref mut fresh4 = (*graph.offset(i as isize)).ewgts;
            *fresh4 = old_weights;
            old_weights = old_weights.offset((*graph.offset(i as isize)).nedges as isize);
            i += 1;
        }
    }
}
