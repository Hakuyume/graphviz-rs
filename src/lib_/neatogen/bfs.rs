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
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
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
pub unsafe extern "C" fn bfs(
    mut vertex: libc::c_int,
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut dist: *mut DistType,
    mut Q: *mut Queue,
) {
    let mut i: libc::c_int = 0;
    let mut closestVertex: libc::c_int = 0;
    let mut neighbor: libc::c_int = 0;
    let mut closestDist: DistType = 2147483647 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *dist.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    *dist.offset(vertex as isize) = 0 as libc::c_int;
    initQueue(Q, vertex);
    if ((*graph.offset(0 as libc::c_int as isize)).ewgts).is_null() {
        while deQueue(Q, &mut closestVertex) {
            closestDist = *dist.offset(closestVertex as isize);
            i = 1 as libc::c_int;
            while i < (*graph.offset(closestVertex as isize)).nedges {
                neighbor = *((*graph.offset(closestVertex as isize)).edges).offset(i as isize);
                if (*dist.offset(neighbor as isize) as libc::c_double) < -0.5f64 {
                    *dist.offset(neighbor as isize) = closestDist + 1 as libc::c_int;
                    enQueue(Q, neighbor);
                }
                i += 1;
            }
        }
    } else {
        while deQueue(Q, &mut closestVertex) {
            closestDist = *dist.offset(closestVertex as isize);
            i = 1 as libc::c_int;
            while i < (*graph.offset(closestVertex as isize)).nedges {
                neighbor = *((*graph.offset(closestVertex as isize)).edges).offset(i as isize);
                if (*dist.offset(neighbor as isize) as libc::c_double) < -0.5f64 {
                    *dist.offset(neighbor as isize) = closestDist
                        + *((*graph.offset(closestVertex as isize)).ewgts).offset(i as isize)
                            as DistType;
                    enQueue(Q, neighbor);
                }
                i += 1;
            }
        }
    }
    i = 0 as libc::c_int;
    while i < n {
        if (*dist.offset(i as isize) as libc::c_double) < -0.5f64 {
            *dist.offset(i as isize) = closestDist + 10 as libc::c_int;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bfs_bounded(
    mut vertex: libc::c_int,
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut dist: *mut DistType,
    mut Q: *mut Queue,
    mut bound: libc::c_int,
    mut visited_nodes: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num_visit: libc::c_int = 0;
    let mut closestVertex: libc::c_int = 0;
    let mut neighbor: libc::c_int = 0;
    let mut closestDist: DistType = 0;
    *dist.offset(vertex as isize) = 0 as libc::c_int;
    initQueue(Q, vertex);
    num_visit = 0 as libc::c_int;
    while deQueue(Q, &mut closestVertex) {
        closestDist = *dist.offset(closestVertex as isize);
        if closestDist > bound {
            *dist.offset(closestVertex as isize) = -(1 as libc::c_int);
            break;
        } else {
            let fresh0 = num_visit;
            num_visit = num_visit + 1;
            *visited_nodes.offset(fresh0 as isize) = closestVertex;
            i = 1 as libc::c_int;
            while i < (*graph.offset(closestVertex as isize)).nedges {
                neighbor = *((*graph.offset(closestVertex as isize)).edges).offset(i as isize);
                if (*dist.offset(neighbor as isize) as libc::c_double) < -0.5f64 {
                    *dist.offset(neighbor as isize) = closestDist + 1 as libc::c_int;
                    enQueue(Q, neighbor);
                }
                i += 1;
            }
        }
    }
    while deQueue(Q, &mut closestVertex) {
        *dist.offset(closestVertex as isize) = -(1 as libc::c_int);
    }
    *dist.offset(vertex as isize) = -(1 as libc::c_int);
    return num_visit;
}
#[no_mangle]
pub unsafe extern "C" fn mkQueue(mut qp: *mut Queue, mut size: libc::c_int) {
    let ref mut fresh1 = (*qp).data;
    *fresh1 = gcalloc(
        size as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    (*qp).queueSize = size;
    let ref mut fresh2 = (*qp).end;
    *fresh2 = 0 as libc::c_int;
    (*qp).start = *fresh2;
}
#[no_mangle]
pub unsafe extern "C" fn freeQueue(mut qp: *mut Queue) {
    free((*qp).data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn initQueue(mut qp: *mut Queue, mut startVertex: libc::c_int) {
    *((*qp).data).offset(0 as libc::c_int as isize) = startVertex;
    (*qp).start = 0 as libc::c_int;
    (*qp).end = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn deQueue(mut qp: *mut Queue, mut vertex: *mut libc::c_int) -> bool {
    if (*qp).start >= (*qp).end {
        return 0 as libc::c_int != 0;
    }
    let ref mut fresh3 = (*qp).start;
    let fresh4 = *fresh3;
    *fresh3 = *fresh3 + 1;
    *vertex = *((*qp).data).offset(fresh4 as isize);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn enQueue(mut qp: *mut Queue, mut vertex: libc::c_int) -> bool {
    if (*qp).end >= (*qp).queueSize {
        return 0 as libc::c_int != 0;
    }
    let ref mut fresh5 = (*qp).end;
    let fresh6 = *fresh5;
    *fresh5 = *fresh5 + 1;
    *((*qp).data).offset(fresh6 as isize) = vertex;
    return 1 as libc::c_int != 0;
}
