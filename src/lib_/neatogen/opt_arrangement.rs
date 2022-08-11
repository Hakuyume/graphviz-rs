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
    fn init_vec_orth1(n: libc::c_int, vec: *mut libc::c_double);
    fn conjugate_gradient(
        _: *mut vtx_data,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_double,
        _: libc::c_int,
    ) -> libc::c_int;
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
unsafe extern "C" fn construct_b(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut b: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut b_i: libc::c_double = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        b_i = 0 as libc::c_int as libc::c_double;
        if !((*graph.offset(0 as libc::c_int as isize)).edists).is_null() {
            j = 1 as libc::c_int;
            while j < (*graph.offset(i as isize)).nedges {
                b_i += (*((*graph.offset(i as isize)).ewgts).offset(j as isize)
                    * *((*graph.offset(i as isize)).edists).offset(j as isize))
                    as libc::c_double;
                j += 1;
            }
            *b.offset(i as isize) = b_i;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn compute_y_coords(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut y_coords: *mut libc::c_double,
    mut max_iterations: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut b: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut tol: libc::c_double = 1e-3f64;
    let mut nedges: libc::c_int = 0 as libc::c_int;
    let mut uniform_weights: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut old_ewgts: *mut libc::c_float = (*graph.offset(0 as libc::c_int as isize)).ewgts;
    construct_b(graph, n, b);
    init_vec_orth1(n, y_coords);
    i = 0 as libc::c_int;
    while i < n {
        nedges += (*graph.offset(i as isize)).nedges;
        i += 1;
    }
    uniform_weights = gcalloc(
        nedges as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh0 = (*graph.offset(i as isize)).ewgts;
        *fresh0 = uniform_weights;
        *uniform_weights.offset(0 as libc::c_int as isize) =
            -((*graph.offset(i as isize)).nedges - 1 as libc::c_int) as libc::c_float;
        j = 1 as libc::c_int;
        while j < (*graph.offset(i as isize)).nedges {
            *uniform_weights.offset(j as isize) = 1 as libc::c_int as libc::c_float;
            j += 1;
        }
        uniform_weights = uniform_weights.offset((*graph.offset(i as isize)).nedges as isize);
        i += 1;
    }
    if conjugate_gradient(graph, y_coords, b, n, tol, max_iterations) < 0 as libc::c_int {
        rv = 1 as libc::c_int;
    }
    free((*graph.offset(0 as libc::c_int as isize)).ewgts as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh1 = (*graph.offset(i as isize)).ewgts;
        *fresh1 = old_ewgts;
        old_ewgts = old_ewgts.offset((*graph.offset(i as isize)).nedges as isize);
        i += 1;
    }
    free(b as *mut libc::c_void);
    return rv;
}
