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
    fn free(_: *mut libc::c_void);
    fn rand() -> libc::c_int;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn dijkstra(_: libc::c_int, _: *mut vtx_data, _: libc::c_int, _: *mut DistType);
    fn mkQueue(_: *mut Queue, _: libc::c_int);
    fn bfs(_: libc::c_int, _: *mut vtx_data, _: libc::c_int, _: *mut DistType, _: *mut Queue);
    fn compute_new_weights(graph: *mut vtx_data, n: libc::c_int);
    fn restore_old_weights(graph: *mut vtx_data, n: libc::c_int, old_weights: *mut libc::c_float);
}
pub type size_t = libc::c_ulong;
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
pub unsafe extern "C" fn embed_graph(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut Coords: *mut *mut *mut DistType,
    mut reweight_graph: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut node: libc::c_int = 0;
    let mut storage: *mut DistType = gcalloc(
        (n * dim) as size_t,
        ::std::mem::size_of::<DistType>() as libc::c_ulong,
    ) as *mut DistType;
    let mut coords: *mut *mut DistType = *Coords;
    let mut dist: *mut DistType = gcalloc(
        n as size_t,
        ::std::mem::size_of::<DistType>() as libc::c_ulong,
    ) as *mut DistType;
    let mut old_weights: *mut libc::c_float = (*graph.offset(0 as libc::c_int as isize)).ewgts;
    let mut Q: Queue = Queue {
        data: 0 as *mut libc::c_int,
        queueSize: 0,
        end: 0,
        start: 0,
    };
    let mut max_dist: DistType = 0 as libc::c_int;
    if !coords.is_null() {
        free(*coords.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        free(coords as *mut libc::c_void);
    }
    coords = gcalloc(
        dim as size_t,
        ::std::mem::size_of::<*mut DistType>() as libc::c_ulong,
    ) as *mut *mut DistType;
    *Coords = coords;
    i = 0 as libc::c_int;
    while i < dim {
        let ref mut fresh0 = *coords.offset(i as isize);
        *fresh0 = storage.offset((i * n) as isize);
        i += 1;
    }
    if reweight_graph != 0 {
        compute_new_weights(graph, n);
    }
    node = rand() % n;
    mkQueue(&mut Q, n);
    if reweight_graph != 0 {
        dijkstra(node, graph, n, *coords.offset(0 as libc::c_int as isize));
    } else {
        bfs(
            node,
            graph,
            n,
            *coords.offset(0 as libc::c_int as isize),
            &mut Q,
        );
    }
    i = 0 as libc::c_int;
    while i < n {
        *dist.offset(i as isize) = *(*coords.offset(0 as libc::c_int as isize)).offset(i as isize);
        if *dist.offset(i as isize) > max_dist {
            node = i;
            max_dist = *dist.offset(i as isize);
        }
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < dim {
        if reweight_graph != 0 {
            dijkstra(node, graph, n, *coords.offset(i as isize));
        } else {
            bfs(node, graph, n, *coords.offset(i as isize), &mut Q);
        }
        max_dist = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < n {
            *dist.offset(j as isize) =
                if *dist.offset(j as isize) < *(*coords.offset(i as isize)).offset(j as isize) {
                    *dist.offset(j as isize)
                } else {
                    *(*coords.offset(i as isize)).offset(j as isize)
                };
            if *dist.offset(j as isize) > max_dist {
                node = j;
                max_dist = *dist.offset(j as isize);
            }
            j += 1;
        }
        i += 1;
    }
    free(dist as *mut libc::c_void);
    if reweight_graph != 0 {
        restore_old_weights(graph, n, old_weights);
    }
}
#[no_mangle]
pub unsafe extern "C" fn center_coordinate(
    mut coords: *mut *mut DistType,
    mut n: libc::c_int,
    mut dim: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
    let mut avg: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < dim {
        sum = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int;
        while j < n {
            sum += *(*coords.offset(i as isize)).offset(j as isize) as libc::c_double;
            j += 1;
        }
        avg = sum / n as libc::c_double;
        j = 0 as libc::c_int;
        while j < n {
            let ref mut fresh1 = *(*coords.offset(i as isize)).offset(j as isize);
            *fresh1 -= avg as DistType;
            j += 1;
        }
        i += 1;
    }
}
