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
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    fn free(_: *mut libc::c_void);
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn orthog1f(n: libc::c_int, vec: *mut libc::c_float);
    fn set_vector_valf(n: libc::c_int, val: libc::c_float, result: *mut libc::c_float);
    fn quicksort_placef(_: *mut libc::c_float, _: *mut libc::c_int, _: libc::c_int, _: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMajEnv {
    pub A: *mut *mut libc::c_float,
    pub n: libc::c_int,
    pub lev: *mut libc::c_int,
    pub iArray1: *mut libc::c_int,
    pub iArray2: *mut libc::c_int,
    pub iArray3: *mut libc::c_int,
    pub iArray4: *mut libc::c_int,
    pub fArray1: *mut libc::c_float,
    pub fArray2: *mut libc::c_float,
    pub fArray3: *mut libc::c_float,
    pub fArray4: *mut libc::c_float,
    pub A_r: *mut libc::c_float,
    pub ordering: *mut libc::c_int,
    pub levels: *mut libc::c_int,
    pub num_levels: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn unpackMatrix(
    mut packedMat: *mut libc::c_float,
    mut n: libc::c_int,
) -> *mut *mut libc::c_float {
    let mut mat: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    mat = gcalloc(
        n as size_t,
        ::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong,
    ) as *mut *mut libc::c_float;
    let ref mut fresh0 = *mat.offset(0 as libc::c_int as isize);
    *fresh0 = gcalloc(
        (n * n) as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    set_vector_valf(
        n * n,
        0 as libc::c_int as libc::c_float,
        *mat.offset(0 as libc::c_int as isize),
    );
    i = 1 as libc::c_int;
    while i < n {
        let ref mut fresh1 = *mat.offset(i as isize);
        *fresh1 = (*mat.offset(0 as libc::c_int as isize)).offset((i * n) as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while i < n {
        j = i;
        while j < n {
            let ref mut fresh2 = *(*mat.offset(i as isize)).offset(j as isize);
            *fresh2 = *packedMat.offset(k as isize);
            *(*mat.offset(j as isize)).offset(i as isize) = *fresh2;
            j += 1;
            k += 1;
        }
        i += 1;
    }
    return mat;
}
unsafe extern "C" fn ensureMonotonicOrdering(
    mut place_0: *mut libc::c_float,
    mut n: libc::c_int,
    mut ordering: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut node: libc::c_int = 0;
    let mut lower_bound: libc::c_float =
        *place_0.offset(*ordering.offset(0 as libc::c_int as isize) as isize);
    i = 1 as libc::c_int;
    while i < n {
        node = *ordering.offset(i as isize);
        if *place_0.offset(node as isize) < lower_bound {
            *place_0.offset(node as isize) = lower_bound;
        }
        lower_bound = *place_0.offset(node as isize);
        i += 1;
    }
}
unsafe extern "C" fn ensureMonotonicOrderingWithGaps(
    mut place_0: *mut libc::c_float,
    mut n: libc::c_int,
    mut ordering: *mut libc::c_int,
    mut levels: *mut libc::c_int,
    mut num_levels: libc::c_int,
    mut levels_gap: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut node: libc::c_int = 0;
    let mut level: libc::c_int = 0;
    let mut max_in_level: libc::c_int = 0;
    let mut lower_bound: libc::c_float = -1e9f64 as libc::c_float;
    level = -(1 as libc::c_int);
    max_in_level = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if i >= max_in_level {
            level += 1;
            if level == num_levels {
                max_in_level = n;
            } else {
                max_in_level = *levels.offset(level as isize);
            }
            lower_bound = if i > 0 as libc::c_int {
                *place_0.offset(*ordering.offset((i - 1 as libc::c_int) as isize) as isize)
                    + levels_gap
            } else {
                -1e9f64 as libc::c_float
            };
            quicksort_placef(place_0, ordering, i, max_in_level - 1 as libc::c_int);
        }
        node = *ordering.offset(i as isize);
        if *place_0.offset(node as isize) < lower_bound {
            *place_0.offset(node as isize) = lower_bound;
        }
        i += 1;
    }
}
unsafe extern "C" fn computeHierarchyBoundaries(
    mut place_0: *mut libc::c_float,
    mut n: libc::c_int,
    mut ordering: *mut libc::c_int,
    mut levels: *mut libc::c_int,
    mut num_levels: libc::c_int,
    mut hierarchy_boundaries: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_levels {
        *hierarchy_boundaries.offset(i as isize) = *place_0.offset(
            *ordering.offset((*levels.offset(i as isize) - 1 as libc::c_int) as isize) as isize,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn constrained_majorization_new(
    mut e: *mut CMajEnv,
    mut b: *mut libc::c_float,
    mut coords: *mut *mut libc::c_float,
    mut cur_axis: libc::c_int,
    mut dims: libc::c_int,
    mut max_iterations: libc::c_int,
    mut hierarchy_boundaries: *mut libc::c_float,
    mut levels_gap: libc::c_float,
) -> libc::c_int {
    let mut n: libc::c_int = (*e).n;
    let mut place_0: *mut libc::c_float = *coords.offset(cur_axis as isize);
    let mut lap: *mut *mut libc::c_float = (*e).A;
    let mut ordering: *mut libc::c_int = (*e).ordering;
    let mut levels: *mut libc::c_int = (*e).levels;
    let mut num_levels: libc::c_int = (*e).num_levels;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut new_place_i: libc::c_float = 0.;
    let mut converged: bool = 0 as libc::c_int != 0;
    let mut upper_bound: libc::c_float = 0.;
    let mut lower_bound: libc::c_float = 0.;
    let mut node: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut cur_place: libc::c_float = 0.;
    let mut des_place_block: libc::c_float = 0.;
    let mut block_deg: libc::c_float = 0.;
    let mut toBlockConnectivity: libc::c_float = 0.;
    let mut lap_node: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut block_len: libc::c_int = 0;
    let mut first_next_level: libc::c_int = 0;
    let mut level: libc::c_int = -(1 as libc::c_int);
    let mut max_in_level: libc::c_int = 0 as libc::c_int;
    let mut desired_place: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut prefix_desired_place: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut suffix_desired_place: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut block: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut lev: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut counter: libc::c_int = 0;
    if max_iterations <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if levels_gap != 0 as libc::c_int as libc::c_float {
        return constrained_majorization_new_with_gaps(
            e,
            b,
            coords,
            cur_axis,
            dims,
            max_iterations,
            hierarchy_boundaries,
            levels_gap,
        );
    }
    ensureMonotonicOrdering(place_0, n, ordering);
    desired_place = (*e).fArray1;
    prefix_desired_place = (*e).fArray2;
    suffix_desired_place = (*e).fArray3;
    block = (*e).iArray1;
    lev = (*e).iArray2;
    i = 0 as libc::c_int;
    while i < n {
        if i >= max_in_level {
            level += 1;
            if level == num_levels {
                max_in_level = n;
            } else {
                max_in_level = *levels.offset(level as isize);
            }
        }
        node = *ordering.offset(i as isize);
        *lev.offset(node as isize) = level;
        i += 1;
    }
    counter = 0 as libc::c_int;
    while counter < max_iterations && !converged {
        converged = 1 as libc::c_int != 0;
        lower_bound = -1e9f64 as libc::c_float;
        left = 0 as libc::c_int;
        while left < n {
            let mut best_i: libc::c_int = 0;
            let mut max_movement: libc::c_double = 0.;
            let mut movement: libc::c_double = 0.;
            let mut prefix_des_place: libc::c_float = 0.;
            let mut suffix_des_place: libc::c_float = 0.;
            cur_place = *place_0.offset(*ordering.offset(left as isize) as isize);
            right = left + 1 as libc::c_int;
            while right < n {
                if *place_0.offset(*ordering.offset(right as isize) as isize) != cur_place {
                    break;
                }
                right += 1;
            }
            i = left;
            while i < right {
                node = *ordering.offset(i as isize);
                new_place_i = -*b.offset(node as isize);
                lap_node = *lap.offset(node as isize);
                j = 0 as libc::c_int;
                while j < n {
                    if !(j == node) {
                        new_place_i += *lap_node.offset(j as isize) * *place_0.offset(j as isize);
                    }
                    j += 1;
                }
                *desired_place.offset(node as isize) =
                    new_place_i / -*lap_node.offset(node as isize);
                i += 1;
            }
            block_len = 0 as libc::c_int;
            i = left;
            while i < right {
                level = *lev.offset(*ordering.offset(i as isize) as isize);
                if level == num_levels {
                    first_next_level = right;
                } else {
                    first_next_level = if right < *levels.offset(level as isize) {
                        right
                    } else {
                        *levels.offset(level as isize)
                    };
                }
                j = i;
                while j < first_next_level {
                    node = *ordering.offset(j as isize);
                    if *desired_place.offset(node as isize) < cur_place {
                        let fresh3 = block_len;
                        block_len = block_len + 1;
                        *block.offset(fresh3 as isize) = node;
                    }
                    j += 1;
                }
                j = i;
                while j < first_next_level {
                    node = *ordering.offset(j as isize);
                    if *desired_place.offset(node as isize) == cur_place {
                        let fresh4 = block_len;
                        block_len = block_len + 1;
                        *block.offset(fresh4 as isize) = node;
                    }
                    j += 1;
                }
                j = i;
                while j < first_next_level {
                    node = *ordering.offset(j as isize);
                    if *desired_place.offset(node as isize) > cur_place {
                        let fresh5 = block_len;
                        block_len = block_len + 1;
                        *block.offset(fresh5 as isize) = node;
                    }
                    j += 1;
                }
                i = first_next_level;
            }
            des_place_block = 0 as libc::c_int as libc::c_float;
            block_deg = 0 as libc::c_int as libc::c_float;
            i = 0 as libc::c_int;
            while i < block_len {
                node = *block.offset(i as isize);
                toBlockConnectivity = 0 as libc::c_int as libc::c_float;
                lap_node = *lap.offset(node as isize);
                j = 0 as libc::c_int;
                while j < i {
                    toBlockConnectivity -= *lap_node.offset(*block.offset(j as isize) as isize);
                    j += 1;
                }
                toBlockConnectivity *= 2 as libc::c_int as libc::c_float;
                des_place_block = (block_deg * des_place_block
                    + -*lap_node.offset(node as isize) * *desired_place.offset(node as isize)
                    + toBlockConnectivity * cur_place)
                    / (block_deg - *lap_node.offset(node as isize) + toBlockConnectivity);
                *prefix_desired_place.offset(i as isize) = des_place_block;
                block_deg += toBlockConnectivity - *lap_node.offset(node as isize);
                i += 1;
            }
            des_place_block = 0 as libc::c_int as libc::c_float;
            block_deg = 0 as libc::c_int as libc::c_float;
            i = block_len - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                node = *block.offset(i as isize);
                toBlockConnectivity = 0 as libc::c_int as libc::c_float;
                lap_node = *lap.offset(node as isize);
                j = i + 1 as libc::c_int;
                while j < block_len {
                    toBlockConnectivity -= *lap_node.offset(*block.offset(j as isize) as isize);
                    j += 1;
                }
                toBlockConnectivity *= 2 as libc::c_int as libc::c_float;
                des_place_block = (block_deg * des_place_block
                    + -*lap_node.offset(node as isize) * *desired_place.offset(node as isize)
                    + toBlockConnectivity * cur_place)
                    / (block_deg - *lap_node.offset(node as isize) + toBlockConnectivity);
                *suffix_desired_place.offset(i as isize) = des_place_block;
                block_deg += toBlockConnectivity - *lap_node.offset(node as isize);
                i -= 1;
            }
            best_i = -(1 as libc::c_int);
            max_movement = 0 as libc::c_int as libc::c_double;
            i = 0 as libc::c_int;
            while i < block_len {
                suffix_des_place = *suffix_desired_place.offset(i as isize);
                prefix_des_place = if i > 0 as libc::c_int {
                    *prefix_desired_place.offset((i - 1 as libc::c_int) as isize)
                } else {
                    suffix_des_place
                };
                if suffix_des_place < prefix_des_place {
                    if suffix_des_place < cur_place {
                        if prefix_des_place > cur_place {
                            prefix_des_place = cur_place;
                        }
                        suffix_des_place = prefix_des_place;
                    } else if prefix_des_place > cur_place {
                        prefix_des_place = suffix_des_place;
                    }
                }
                movement = (block_len - i) as libc::c_double
                    * fabs((suffix_des_place - cur_place) as libc::c_double)
                    + i as libc::c_double * fabs((prefix_des_place - cur_place) as libc::c_double);
                if movement > max_movement {
                    max_movement = movement;
                    best_i = i;
                }
                i += 1;
            }
            if best_i >= 0 as libc::c_int {
                suffix_des_place = *suffix_desired_place.offset(best_i as isize);
                prefix_des_place = if best_i > 0 as libc::c_int {
                    *prefix_desired_place.offset((best_i - 1 as libc::c_int) as isize)
                } else {
                    suffix_des_place
                };
                if right >= n {
                    upper_bound = 1e9f64 as libc::c_float;
                } else {
                    upper_bound = *place_0.offset(*ordering.offset(right as isize) as isize);
                }
                suffix_des_place = if suffix_des_place < upper_bound {
                    suffix_des_place
                } else {
                    upper_bound
                };
                prefix_des_place = if prefix_des_place > lower_bound {
                    prefix_des_place
                } else {
                    lower_bound
                };
                if suffix_des_place < prefix_des_place {
                    if suffix_des_place < cur_place {
                        if prefix_des_place > cur_place {
                            prefix_des_place = cur_place;
                        }
                        suffix_des_place = prefix_des_place;
                    } else if prefix_des_place > cur_place {
                        prefix_des_place = suffix_des_place;
                    }
                }
                i = 0 as libc::c_int;
                while i < best_i {
                    *place_0.offset(*block.offset(i as isize) as isize) = prefix_des_place;
                    i += 1;
                }
                i = best_i;
                while i < block_len {
                    *place_0.offset(*block.offset(i as isize) as isize) = suffix_des_place;
                    i += 1;
                }
                lower_bound = suffix_des_place;
                i = left;
                while i < right {
                    *ordering.offset(i as isize) = *block.offset((i - left) as isize);
                    i += 1;
                }
                converged = converged as libc::c_int != 0
                    && fabs((prefix_des_place - cur_place) as libc::c_double) < 1e-2f64
                    && fabs((suffix_des_place - cur_place) as libc::c_double) < 1e-2f64;
            } else {
                lower_bound = cur_place;
            }
            left = right;
        }
        counter += 1;
    }
    computeHierarchyBoundaries(
        place_0,
        n,
        ordering,
        levels,
        num_levels,
        hierarchy_boundaries,
    );
    return counter;
}
static mut place: *mut libc::c_float = 0 as *const libc::c_float as *mut libc::c_float;
unsafe extern "C" fn compare_incr(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    if *place.offset(*(a as *const libc::c_int) as isize)
        > *place.offset(*(b as *const libc::c_int) as isize)
    {
        return 1 as libc::c_int;
    } else {
        if *place.offset(*(a as *const libc::c_int) as isize)
            < *place.offset(*(b as *const libc::c_int) as isize)
        {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn constrained_majorization_gradient_projection(
    mut e: *mut CMajEnv,
    mut b: *mut libc::c_float,
    mut coords: *mut *mut libc::c_float,
    mut ndims: libc::c_int,
    mut cur_axis: libc::c_int,
    mut max_iterations: libc::c_int,
    mut hierarchy_boundaries: *mut libc::c_float,
    mut levels_gap: libc::c_float,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut counter: libc::c_int = 0;
    let mut ordering: *mut libc::c_int = (*e).ordering;
    let mut levels: *mut libc::c_int = (*e).levels;
    let mut num_levels: libc::c_int = (*e).num_levels;
    let mut converged: bool = 0 as libc::c_int != 0;
    let mut g: *mut libc::c_float = (*e).fArray1;
    let mut old_place: *mut libc::c_float = (*e).fArray2;
    let mut d: *mut libc::c_float = (*e).fArray4;
    let mut test: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut tmptest: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut beta: libc::c_float = 0.;
    if max_iterations == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    place = *coords.offset(cur_axis as isize);
    counter = 0 as libc::c_int;
    while counter < max_iterations && !converged {
        let mut alpha: libc::c_float = 0.;
        let mut numerator: libc::c_float = 0 as libc::c_int as libc::c_float;
        let mut denominator: libc::c_float = 0 as libc::c_int as libc::c_float;
        let mut r: libc::c_float = 0.;
        converged = 1 as libc::c_int != 0;
        i = 0 as libc::c_int;
        while i < (*e).n {
            *old_place.offset(i as isize) = *place.offset(i as isize);
            *g.offset(i as isize) = 2 as libc::c_int as libc::c_float * *b.offset(i as isize);
            j = 0 as libc::c_int;
            while j < (*e).n {
                *g.offset(i as isize) -= 2 as libc::c_int as libc::c_float
                    * *(*((*e).A).offset(i as isize)).offset(j as isize)
                    * *place.offset(j as isize);
                j += 1;
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < (*e).n {
            numerator += *g.offset(i as isize) * *g.offset(i as isize);
            r = 0 as libc::c_int as libc::c_float;
            j = 0 as libc::c_int;
            while j < (*e).n {
                r += 2 as libc::c_int as libc::c_float
                    * *(*((*e).A).offset(i as isize)).offset(j as isize)
                    * *g.offset(j as isize);
                j += 1;
            }
            denominator -= r * *g.offset(i as isize);
            i += 1;
        }
        alpha = numerator / denominator;
        i = 0 as libc::c_int;
        while i < (*e).n {
            if alpha > 0 as libc::c_int as libc::c_float
                && alpha < 1000 as libc::c_int as libc::c_float
            {
                *place.offset(i as isize) -= alpha * *g.offset(i as isize);
            }
            i += 1;
        }
        if num_levels != 0 {
            qsort(
                ordering as *mut libc::c_void,
                *levels.offset(0 as libc::c_int as isize) as size_t,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                Some(
                    compare_incr
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
        }
        i = 0 as libc::c_int;
        while i < num_levels {
            let mut endOfLevel: libc::c_int = if i == num_levels - 1 as libc::c_int {
                (*e).n
            } else {
                *levels.offset((i + 1 as libc::c_int) as isize)
            };
            let mut ui: libc::c_int = 0;
            let mut li: libc::c_int = 0;
            let mut u: libc::c_int = 0;
            let mut l: libc::c_int = 0;
            qsort(
                ordering.offset(*levels.offset(i as isize) as isize) as *mut libc::c_void,
                (endOfLevel as size_t).wrapping_sub(*levels.offset(i as isize) as libc::c_ulong),
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                Some(
                    compare_incr
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            ui = *levels.offset(i as isize);
            li = ui - 1 as libc::c_int;
            let fresh6 = li;
            li = li - 1;
            l = *ordering.offset(fresh6 as isize);
            let fresh7 = ui;
            ui = ui + 1;
            u = *ordering.offset(fresh7 as isize);
            if *place.offset(l as isize) + levels_gap > *place.offset(u as isize) {
                let mut sum: libc::c_float = *place.offset(l as isize) + *place.offset(u as isize)
                    - levels_gap
                        * (*((*e).lev).offset(l as isize) + *((*e).lev).offset(u as isize))
                            as libc::c_float;
                let mut w: libc::c_float = 2 as libc::c_int as libc::c_float;
                let mut avgPos: libc::c_float = sum / w;
                let mut pos: libc::c_float = 0.;
                let mut finished: bool = false;
                loop {
                    finished = 1 as libc::c_int != 0;
                    if ui < endOfLevel {
                        u = *ordering.offset(ui as isize);
                        pos = *place.offset(u as isize)
                            - levels_gap * *((*e).lev).offset(u as isize) as libc::c_float;
                        if pos < avgPos {
                            ui += 1;
                            w += 1.;
                            sum += pos;
                            avgPos = sum / w;
                            finished = 0 as libc::c_int != 0;
                        }
                    }
                    if li >= 0 as libc::c_int {
                        l = *ordering.offset(li as isize);
                        pos = *place.offset(l as isize)
                            - levels_gap * *((*e).lev).offset(l as isize) as libc::c_float;
                        if pos > avgPos {
                            li -= 1;
                            w += 1.;
                            sum += pos;
                            avgPos = sum / w;
                            finished = 0 as libc::c_int != 0;
                        }
                    }
                    if finished {
                        break;
                    }
                }
                j = li + 1 as libc::c_int;
                while j < ui {
                    *place.offset(*ordering.offset(j as isize) as isize) = avgPos
                        + levels_gap
                            * *((*e).lev).offset(*ordering.offset(j as isize) as isize)
                                as libc::c_float;
                    j += 1;
                }
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < (*e).n {
            *d.offset(i as isize) = *place.offset(i as isize) - *old_place.offset(i as isize);
            i += 1;
        }
        numerator = 0 as libc::c_int as libc::c_float;
        denominator = 0 as libc::c_int as libc::c_float;
        i = 0 as libc::c_int;
        while i < (*e).n {
            numerator += *g.offset(i as isize) * *d.offset(i as isize);
            r = 0 as libc::c_int as libc::c_float;
            j = 0 as libc::c_int;
            while j < (*e).n {
                r += 2 as libc::c_int as libc::c_float
                    * *(*((*e).A).offset(i as isize)).offset(j as isize)
                    * *d.offset(j as isize);
                j += 1;
            }
            denominator += r * *d.offset(i as isize);
            i += 1;
        }
        beta = numerator / denominator;
        i = 0 as libc::c_int;
        while i < (*e).n {
            if beta > 0 as libc::c_int as libc::c_float && (beta as libc::c_double) < 1.0f64 {
                *place.offset(i as isize) =
                    *old_place.offset(i as isize) + beta * *d.offset(i as isize);
            }
            tmptest =
                fabs((*place.offset(i as isize) - *old_place.offset(i as isize)) as libc::c_double)
                    as libc::c_float;
            if test < tmptest {
                test = tmptest;
            }
            i += 1;
        }
        computeHierarchyBoundaries(
            place,
            (*e).n,
            ordering,
            levels,
            num_levels,
            hierarchy_boundaries,
        );
        if test as libc::c_double > 1e-2f64 {
            converged = 0 as libc::c_int != 0;
        }
        counter += 1;
    }
    return counter;
}
#[no_mangle]
pub unsafe extern "C" fn constrained_majorization_new_with_gaps(
    mut e: *mut CMajEnv,
    mut b: *mut libc::c_float,
    mut coords: *mut *mut libc::c_float,
    mut ndims: libc::c_int,
    mut cur_axis: libc::c_int,
    mut max_iterations: libc::c_int,
    mut hierarchy_boundaries: *mut libc::c_float,
    mut levels_gap: libc::c_float,
) -> libc::c_int {
    let mut place_0: *mut libc::c_float = *coords.offset(cur_axis as isize);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = (*e).n;
    let mut lap: *mut *mut libc::c_float = (*e).A;
    let mut ordering: *mut libc::c_int = (*e).ordering;
    let mut levels: *mut libc::c_int = (*e).levels;
    let mut num_levels: libc::c_int = (*e).num_levels;
    let mut new_place_i: libc::c_float = 0.;
    let mut converged: bool = 0 as libc::c_int != 0;
    let mut upper_bound: libc::c_float = 0.;
    let mut lower_bound: libc::c_float = 0.;
    let mut node: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut cur_place: libc::c_float = 0.;
    let mut des_place_block: libc::c_float = 0.;
    let mut block_deg: libc::c_float = 0.;
    let mut toBlockConnectivity: libc::c_float = 0.;
    let mut lap_node: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut desired_place: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut prefix_desired_place: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut suffix_desired_place: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut block: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut block_len: libc::c_int = 0;
    let mut first_next_level: libc::c_int = 0;
    let mut lev: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut level: libc::c_int = -(1 as libc::c_int);
    let mut max_in_level: libc::c_int = 0 as libc::c_int;
    let mut counter: libc::c_int = 0;
    let mut gap: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut target_place: libc::c_float = 0.;
    if max_iterations <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    ensureMonotonicOrderingWithGaps(place_0, n, ordering, levels, num_levels, levels_gap);
    desired_place = (*e).fArray1;
    prefix_desired_place = (*e).fArray2;
    suffix_desired_place = (*e).fArray3;
    block = (*e).iArray1;
    lev = (*e).iArray2;
    i = 0 as libc::c_int;
    while i < n {
        if i >= max_in_level {
            level += 1;
            if level == num_levels {
                max_in_level = n;
            } else {
                max_in_level = *levels.offset(level as isize);
            }
        }
        node = *ordering.offset(i as isize);
        *lev.offset(node as isize) = level;
        i += 1;
    }
    gap = (*e).fArray4;
    counter = 0 as libc::c_int;
    while counter < max_iterations && !converged {
        converged = 1 as libc::c_int != 0;
        lower_bound = -1e9f64 as libc::c_float;
        left = 0 as libc::c_int;
        while left < n {
            let mut best_i: libc::c_int = 0;
            let mut max_movement: libc::c_double = 0.;
            let mut movement: libc::c_double = 0.;
            let mut prefix_des_place: libc::c_float = 0.;
            let mut suffix_des_place: libc::c_float = 0.;
            cur_place = *place_0.offset(*ordering.offset(left as isize) as isize);
            target_place = cur_place;
            *gap.offset(*ordering.offset(left as isize) as isize) =
                0 as libc::c_int as libc::c_float;
            right = left + 1 as libc::c_int;
            while right < n {
                if *lev.offset(right as isize) > *lev.offset((right - 1 as libc::c_int) as isize) {
                    target_place += levels_gap;
                }
                node = *ordering.offset(right as isize);
                if fabs((*place_0.offset(node as isize) - target_place) as libc::c_double) > 1e-9f64
                {
                    break;
                }
                *gap.offset(node as isize) = *place_0.offset(node as isize) - cur_place;
                right += 1;
            }
            i = left;
            while i < right {
                node = *ordering.offset(i as isize);
                new_place_i = -*b.offset(node as isize);
                lap_node = *lap.offset(node as isize);
                j = 0 as libc::c_int;
                while j < n {
                    if !(j == node) {
                        new_place_i += *lap_node.offset(j as isize) * *place_0.offset(j as isize);
                    }
                    j += 1;
                }
                *desired_place.offset(node as isize) =
                    new_place_i / -*lap_node.offset(node as isize) - *gap.offset(node as isize);
                i += 1;
            }
            block_len = 0 as libc::c_int;
            first_next_level = 0 as libc::c_int;
            i = left;
            while i < right {
                level = *lev.offset(*ordering.offset(i as isize) as isize);
                if level == num_levels {
                    first_next_level = right;
                } else {
                    first_next_level = if right < *levels.offset(level as isize) {
                        right
                    } else {
                        *levels.offset(level as isize)
                    };
                }
                j = i;
                while j < first_next_level {
                    node = *ordering.offset(j as isize);
                    if *desired_place.offset(node as isize) < cur_place {
                        let fresh8 = block_len;
                        block_len = block_len + 1;
                        *block.offset(fresh8 as isize) = node;
                    }
                    j += 1;
                }
                j = i;
                while j < first_next_level {
                    node = *ordering.offset(j as isize);
                    if *desired_place.offset(node as isize) == cur_place {
                        let fresh9 = block_len;
                        block_len = block_len + 1;
                        *block.offset(fresh9 as isize) = node;
                    }
                    j += 1;
                }
                j = i;
                while j < first_next_level {
                    node = *ordering.offset(j as isize);
                    if *desired_place.offset(node as isize) > cur_place {
                        let fresh10 = block_len;
                        block_len = block_len + 1;
                        *block.offset(fresh10 as isize) = node;
                    }
                    j += 1;
                }
                i = first_next_level;
            }
            des_place_block = 0 as libc::c_int as libc::c_float;
            block_deg = 0 as libc::c_int as libc::c_float;
            i = 0 as libc::c_int;
            while i < block_len {
                node = *block.offset(i as isize);
                toBlockConnectivity = 0 as libc::c_int as libc::c_float;
                lap_node = *lap.offset(node as isize);
                j = 0 as libc::c_int;
                while j < i {
                    toBlockConnectivity -= *lap_node.offset(*block.offset(j as isize) as isize);
                    j += 1;
                }
                toBlockConnectivity *= 2 as libc::c_int as libc::c_float;
                des_place_block = (block_deg * des_place_block
                    + -*lap_node.offset(node as isize) * *desired_place.offset(node as isize)
                    + toBlockConnectivity * cur_place)
                    / (block_deg - *lap_node.offset(node as isize) + toBlockConnectivity);
                *prefix_desired_place.offset(i as isize) = des_place_block;
                block_deg += toBlockConnectivity - *lap_node.offset(node as isize);
                i += 1;
            }
            if block_len == n {
                *prefix_desired_place.offset((n - 1 as libc::c_int) as isize) = cur_place;
            }
            des_place_block = 0 as libc::c_int as libc::c_float;
            block_deg = 0 as libc::c_int as libc::c_float;
            i = block_len - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                node = *block.offset(i as isize);
                toBlockConnectivity = 0 as libc::c_int as libc::c_float;
                lap_node = *lap.offset(node as isize);
                j = i + 1 as libc::c_int;
                while j < block_len {
                    toBlockConnectivity -= *lap_node.offset(*block.offset(j as isize) as isize);
                    j += 1;
                }
                toBlockConnectivity *= 2 as libc::c_int as libc::c_float;
                des_place_block = (block_deg * des_place_block
                    + -*lap_node.offset(node as isize) * *desired_place.offset(node as isize)
                    + toBlockConnectivity * cur_place)
                    / (block_deg - *lap_node.offset(node as isize) + toBlockConnectivity);
                *suffix_desired_place.offset(i as isize) = des_place_block;
                block_deg += toBlockConnectivity - *lap_node.offset(node as isize);
                i -= 1;
            }
            if block_len == n {
                *suffix_desired_place.offset(0 as libc::c_int as isize) = cur_place;
            }
            best_i = -(1 as libc::c_int);
            max_movement = 0 as libc::c_int as libc::c_double;
            i = 0 as libc::c_int;
            while i < block_len {
                suffix_des_place = *suffix_desired_place.offset(i as isize);
                prefix_des_place = if i > 0 as libc::c_int {
                    *prefix_desired_place.offset((i - 1 as libc::c_int) as isize)
                } else {
                    suffix_des_place
                };
                if suffix_des_place < prefix_des_place {
                    if suffix_des_place < cur_place {
                        if prefix_des_place > cur_place {
                            prefix_des_place = cur_place;
                        }
                        suffix_des_place = prefix_des_place;
                    } else if prefix_des_place > cur_place {
                        prefix_des_place = suffix_des_place;
                    }
                }
                movement = (block_len - i) as libc::c_double
                    * fabs((suffix_des_place - cur_place) as libc::c_double)
                    + i as libc::c_double * fabs((prefix_des_place - cur_place) as libc::c_double);
                if movement > max_movement {
                    max_movement = movement;
                    best_i = i;
                }
                i += 1;
            }
            if best_i >= 0 as libc::c_int {
                suffix_des_place = *suffix_desired_place.offset(best_i as isize);
                prefix_des_place = if best_i > 0 as libc::c_int {
                    *prefix_desired_place.offset((best_i - 1 as libc::c_int) as isize)
                } else {
                    suffix_des_place
                };
                if right >= n {
                    upper_bound = 1e9f64 as libc::c_float;
                } else if *lev.offset(*ordering.offset(right as isize) as isize)
                    > *lev.offset(*ordering.offset((right - 1 as libc::c_int) as isize) as isize)
                {
                    upper_bound =
                        *place_0.offset(*ordering.offset(right as isize) as isize)
                            - levels_gap
                            - *gap.offset(
                                *block.offset((block_len - 1 as libc::c_int) as isize) as isize
                            );
                } else {
                    upper_bound =
                        *place_0.offset(*ordering.offset(right as isize) as isize)
                            - *gap.offset(
                                *block.offset((block_len - 1 as libc::c_int) as isize) as isize
                            );
                }
                suffix_des_place = if suffix_des_place < upper_bound {
                    suffix_des_place
                } else {
                    upper_bound
                };
                prefix_des_place = if prefix_des_place > lower_bound {
                    prefix_des_place
                } else {
                    lower_bound
                };
                if suffix_des_place < prefix_des_place {
                    if suffix_des_place < cur_place {
                        if prefix_des_place > cur_place {
                            prefix_des_place = cur_place;
                        }
                        suffix_des_place = prefix_des_place;
                    } else if prefix_des_place > cur_place {
                        prefix_des_place = suffix_des_place;
                    }
                }
                i = 0 as libc::c_int;
                while i < best_i {
                    *place_0.offset(*block.offset(i as isize) as isize) =
                        prefix_des_place + *gap.offset(*block.offset(i as isize) as isize);
                    i += 1;
                }
                i = best_i;
                while i < block_len {
                    *place_0.offset(*block.offset(i as isize) as isize) =
                        suffix_des_place + *gap.offset(*block.offset(i as isize) as isize);
                    i += 1;
                }
                if right < n
                    && *lev.offset(*ordering.offset(right as isize) as isize)
                        > *lev
                            .offset(*ordering.offset((right - 1 as libc::c_int) as isize) as isize)
                {
                    lower_bound = *place_0
                        .offset(*block.offset((block_len - 1 as libc::c_int) as isize) as isize)
                        + levels_gap;
                } else {
                    lower_bound = *place_0
                        .offset(*block.offset((block_len - 1 as libc::c_int) as isize) as isize);
                }
                i = left;
                while i < right {
                    *ordering.offset(i as isize) = *block.offset((i - left) as isize);
                    i += 1;
                }
                converged = converged as libc::c_int != 0
                    && fabs((prefix_des_place - cur_place) as libc::c_double) < 1e-2f64
                    && fabs((suffix_des_place - cur_place) as libc::c_double) < 1e-2f64;
            } else if right < n
                && *lev.offset(*ordering.offset(right as isize) as isize)
                    > *lev.offset(*ordering.offset((right - 1 as libc::c_int) as isize) as isize)
            {
                lower_bound = *place_0
                    .offset(*block.offset((block_len - 1 as libc::c_int) as isize) as isize)
                    + levels_gap;
            } else {
                lower_bound = *place_0
                    .offset(*block.offset((block_len - 1 as libc::c_int) as isize) as isize);
            }
            left = right;
        }
        orthog1f(n, place_0);
        computeHierarchyBoundaries(
            place_0,
            n,
            ordering,
            levels,
            num_levels,
            hierarchy_boundaries,
        );
        counter += 1;
    }
    return counter;
}
#[no_mangle]
pub unsafe extern "C" fn deleteCMajEnv(mut e: *mut CMajEnv) {
    free(*((*e).A).offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free((*e).A as *mut libc::c_void);
    free((*e).lev as *mut libc::c_void);
    free((*e).fArray1 as *mut libc::c_void);
    free((*e).fArray2 as *mut libc::c_void);
    free((*e).fArray3 as *mut libc::c_void);
    free((*e).fArray4 as *mut libc::c_void);
    free((*e).iArray1 as *mut libc::c_void);
    free((*e).iArray2 as *mut libc::c_void);
    free((*e).iArray3 as *mut libc::c_void);
    free((*e).iArray4 as *mut libc::c_void);
    free(e as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn initConstrainedMajorization(
    mut packedMat: *mut libc::c_float,
    mut n: libc::c_int,
    mut ordering: *mut libc::c_int,
    mut levels: *mut libc::c_int,
    mut num_levels: libc::c_int,
) -> *mut CMajEnv {
    let mut i: libc::c_int = 0;
    let mut level: libc::c_int = -(1 as libc::c_int);
    let mut start_of_level_above: libc::c_int = 0 as libc::c_int;
    let mut e: *mut CMajEnv =
        gmalloc(::std::mem::size_of::<CMajEnv>() as libc::c_ulong) as *mut CMajEnv;
    let ref mut fresh11 = (*e).A;
    *fresh11 = 0 as *mut *mut libc::c_float;
    (*e).n = n;
    let ref mut fresh12 = (*e).ordering;
    *fresh12 = ordering;
    let ref mut fresh13 = (*e).levels;
    *fresh13 = levels;
    (*e).num_levels = num_levels;
    let ref mut fresh14 = (*e).A;
    *fresh14 = unpackMatrix(packedMat, n);
    let ref mut fresh15 = (*e).lev;
    *fresh15 = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*e).n {
        if i >= start_of_level_above {
            level += 1;
            start_of_level_above = if level == num_levels {
                (*e).n
            } else {
                *levels.offset(level as isize)
            };
        }
        *((*e).lev).offset(*ordering.offset(i as isize) as isize) = level;
        i += 1;
    }
    let ref mut fresh16 = (*e).fArray1;
    *fresh16 = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let ref mut fresh17 = (*e).fArray2;
    *fresh17 = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let ref mut fresh18 = (*e).fArray3;
    *fresh18 = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let ref mut fresh19 = (*e).fArray4;
    *fresh19 = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let ref mut fresh20 = (*e).iArray1;
    *fresh20 = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let ref mut fresh21 = (*e).iArray2;
    *fresh21 = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let ref mut fresh22 = (*e).iArray3;
    *fresh22 = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let ref mut fresh23 = (*e).iArray4;
    *fresh23 = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    return e;
}
