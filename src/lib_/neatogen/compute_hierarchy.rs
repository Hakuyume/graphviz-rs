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
    fn compute_y_coords(
        _: *mut vtx_data,
        _: libc::c_int,
        _: *mut libc::c_double,
        _: libc::c_int,
    ) -> libc::c_int;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn quicksort_place(_: *mut libc::c_double, _: *mut libc::c_int, _: libc::c_int, _: libc::c_int);
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
static mut given_levels: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn compute_hierarchy(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut abs_tol: libc::c_double,
    mut relative_tol: libc::c_double,
    mut given_coords: *mut libc::c_double,
    mut orderingp: *mut *mut libc::c_int,
    mut levelsp: *mut *mut libc::c_int,
    mut num_levelsp: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut use_given_levels: libc::c_int = 0 as libc::c_int;
    let mut ordering: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut levels: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tol: libc::c_double = 0.;
    let mut hierarchy_span: libc::c_double = 0.;
    let mut num_levels: libc::c_int = 0;
    if !given_coords.is_null() {
        y = given_coords;
        current_block = 7746791466490516765;
    } else {
        y = gcalloc(
            n as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
        if compute_y_coords(graph, n, y, n) != 0 {
            rv = 1 as libc::c_int;
            current_block = 14575275458752860498;
        } else {
            current_block = 7746791466490516765;
        }
    }
    match current_block {
        7746791466490516765 => {
            ordering = gcalloc(
                n as size_t,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ) as *mut libc::c_int;
            *orderingp = ordering;
            i = 0 as libc::c_int;
            while i < n {
                *ordering.offset(i as isize) = i;
                i += 1;
            }
            quicksort_place(y, ordering, 0 as libc::c_int, n - 1 as libc::c_int);
            if !given_levels.is_null() {
                use_given_levels = (0 as libc::c_int == 0) as libc::c_int;
                i = 0 as libc::c_int;
                while i < n {
                    use_given_levels = (use_given_levels != 0
                        && *given_levels.offset(i as isize) >= 0 as libc::c_int)
                        as libc::c_int;
                    i += 1;
                }
            }
            if use_given_levels != 0 {
                i = 0 as libc::c_int;
                while i < n {
                    *y.offset(i as isize) = *given_levels.offset(i as isize) as libc::c_double;
                    *ordering.offset(i as isize) = i;
                    i += 1;
                }
                quicksort_place(y, ordering, 0 as libc::c_int, n - 1 as libc::c_int);
            }
            hierarchy_span = *y.offset(*ordering.offset((n - 1 as libc::c_int) as isize) as isize)
                - *y.offset(*ordering.offset(0 as libc::c_int as isize) as isize);
            tol = if abs_tol
                > relative_tol * hierarchy_span / (n - 1 as libc::c_int) as libc::c_double
            {
                abs_tol
            } else {
                relative_tol * hierarchy_span / (n - 1 as libc::c_int) as libc::c_double
            };
            num_levels = 0 as libc::c_int;
            i = 1 as libc::c_int;
            while i < n {
                if *y.offset(*ordering.offset(i as isize) as isize)
                    - *y.offset(*ordering.offset((i - 1 as libc::c_int) as isize) as isize)
                    > tol
                {
                    num_levels += 1;
                }
                i += 1;
            }
            *num_levelsp = num_levels;
            if num_levels == 0 as libc::c_int {
                levels = gcalloc(
                    1 as libc::c_int as size_t,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ) as *mut libc::c_int;
                *levelsp = levels;
                *levels.offset(0 as libc::c_int as isize) = n;
            } else {
                let mut count: libc::c_int = 0 as libc::c_int;
                levels = gcalloc(
                    num_levels as size_t,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ) as *mut libc::c_int;
                *levelsp = levels;
                i = 1 as libc::c_int;
                while i < n {
                    if *y.offset(*ordering.offset(i as isize) as isize)
                        - *y.offset(*ordering.offset((i - 1 as libc::c_int) as isize) as isize)
                        > tol
                    {
                        let fresh0 = count;
                        count = count + 1;
                        *levels.offset(fresh0 as isize) = i;
                    }
                    i += 1;
                }
            }
        }
        _ => {}
    }
    if given_coords.is_null() {
        free(y as *mut libc::c_void);
    }
    return rv;
}
