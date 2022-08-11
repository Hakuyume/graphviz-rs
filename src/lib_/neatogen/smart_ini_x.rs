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
    fn rand() -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn compute_apsp(_: *mut vtx_data, _: libc::c_int) -> *mut *mut DistType;
    fn cpvec(_: *mut libc::c_double, _: libc::c_int, _: libc::c_int, _: *mut libc::c_double);
    fn dot(
        _: *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_double,
    ) -> libc::c_double;
    fn scadd(
        _: *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_double,
        _: *mut libc::c_double,
    );
    fn vecscale(
        _: *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_double,
        _: *mut libc::c_double,
    );
    fn norm(_: *mut libc::c_double, _: libc::c_int, _: libc::c_int) -> libc::c_double;
    fn conjugate_gradient_f(
        _: *mut *mut libc::c_float,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_double,
        _: libc::c_int,
        _: bool,
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
pub type DistType = libc::c_int;
unsafe extern "C" fn standardize(mut orthog: *mut libc::c_double, mut nvtxs: libc::c_int) {
    let mut len: libc::c_double = 0.;
    let mut avg: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nvtxs {
        avg += *orthog.offset(i as isize);
        i += 1;
    }
    avg /= nvtxs as libc::c_double;
    i = 0 as libc::c_int;
    while i < nvtxs {
        *orthog.offset(i as isize) -= avg;
        i += 1;
    }
    len = norm(orthog, 0 as libc::c_int, nvtxs - 1 as libc::c_int);
    vecscale(
        orthog,
        0 as libc::c_int,
        nvtxs - 1 as libc::c_int,
        1.0f64 / len,
        orthog,
    );
}
unsafe extern "C" fn mat_mult_vec_orthog(
    mut mat: *mut *mut libc::c_float,
    mut dim1: libc::c_int,
    mut dim2: libc::c_int,
    mut vec: *mut libc::c_double,
    mut result: *mut libc::c_double,
    mut orthog: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < dim1 {
        sum = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int;
        while j < dim2 {
            sum += *(*mat.offset(i as isize)).offset(j as isize) as libc::c_double
                * *vec.offset(j as isize);
            j += 1;
        }
        *result.offset(i as isize) = sum;
        i += 1;
    }
    if !orthog.is_null() {
        let mut alpha: libc::c_double =
            -dot(result, 0 as libc::c_int, dim1 - 1 as libc::c_int, orthog);
        scadd(
            result,
            0 as libc::c_int,
            dim1 - 1 as libc::c_int,
            alpha,
            orthog,
        );
    }
}
unsafe extern "C" fn power_iteration_orthog(
    mut square_mat: *mut *mut libc::c_float,
    mut n: libc::c_int,
    mut neigs: libc::c_int,
    mut eigs: *mut *mut libc::c_double,
    mut evals: *mut libc::c_double,
    mut orthog: *mut libc::c_double,
    mut p_iteration_threshold: libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp_vec: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut last_vec: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut curr_vector: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut len: libc::c_double = 0.;
    let mut angle: libc::c_double = 0.;
    let mut alpha: libc::c_double = 0.;
    let mut iteration: libc::c_int = 0;
    let mut largest_index: libc::c_int = 0;
    let mut largest_eval: libc::c_double = 0.;
    let mut tol: libc::c_double = 1 as libc::c_int as libc::c_double - p_iteration_threshold;
    if neigs >= n {
        neigs = n;
    }
    i = 0 as libc::c_int;
    's_50: while i < neigs {
        curr_vector = *eigs.offset(i as isize);
        loop {
            j = 0 as libc::c_int;
            while j < n {
                *curr_vector.offset(j as isize) = (rand() % 100 as libc::c_int) as libc::c_double;
                j += 1;
            }
            if !orthog.is_null() {
                alpha = -dot(orthog, 0 as libc::c_int, n - 1 as libc::c_int, curr_vector);
                scadd(
                    curr_vector,
                    0 as libc::c_int,
                    n - 1 as libc::c_int,
                    alpha,
                    orthog,
                );
            }
            j = 0 as libc::c_int;
            while j < i {
                alpha = -dot(
                    *eigs.offset(j as isize),
                    0 as libc::c_int,
                    n - 1 as libc::c_int,
                    curr_vector,
                );
                scadd(
                    curr_vector,
                    0 as libc::c_int,
                    n - 1 as libc::c_int,
                    alpha,
                    *eigs.offset(j as isize),
                );
                j += 1;
            }
            len = norm(curr_vector, 0 as libc::c_int, n - 1 as libc::c_int);
            if !(len < 1e-10f64) {
                break;
            }
        }
        vecscale(
            curr_vector,
            0 as libc::c_int,
            n - 1 as libc::c_int,
            1.0f64 / len,
            curr_vector,
        );
        iteration = 0 as libc::c_int;
        loop {
            iteration += 1;
            cpvec(
                last_vec,
                0 as libc::c_int,
                n - 1 as libc::c_int,
                curr_vector,
            );
            mat_mult_vec_orthog(square_mat, n, n, curr_vector, tmp_vec, orthog);
            cpvec(curr_vector, 0 as libc::c_int, n - 1 as libc::c_int, tmp_vec);
            j = 0 as libc::c_int;
            while j < i {
                alpha = -dot(
                    *eigs.offset(j as isize),
                    0 as libc::c_int,
                    n - 1 as libc::c_int,
                    curr_vector,
                );
                scadd(
                    curr_vector,
                    0 as libc::c_int,
                    n - 1 as libc::c_int,
                    alpha,
                    *eigs.offset(j as isize),
                );
                j += 1;
            }
            len = norm(curr_vector, 0 as libc::c_int, n - 1 as libc::c_int);
            if len < 1e-10f64 {
                break 's_50;
            }
            vecscale(
                curr_vector,
                0 as libc::c_int,
                n - 1 as libc::c_int,
                1.0f64 / len,
                curr_vector,
            );
            angle = dot(
                curr_vector,
                0 as libc::c_int,
                n - 1 as libc::c_int,
                last_vec,
            );
            if !(fabs(angle) < tol) {
                break;
            }
        }
        *evals.offset(i as isize) = angle * len;
        i += 1;
    }
    while i < neigs {
        curr_vector = *eigs.offset(i as isize);
        j = 0 as libc::c_int;
        while j < n {
            *curr_vector.offset(j as isize) = (rand() % 100 as libc::c_int) as libc::c_double;
            j += 1;
        }
        j = 0 as libc::c_int;
        while j < i {
            alpha = -dot(
                *eigs.offset(j as isize),
                0 as libc::c_int,
                n - 1 as libc::c_int,
                curr_vector,
            );
            scadd(
                curr_vector,
                0 as libc::c_int,
                n - 1 as libc::c_int,
                alpha,
                *eigs.offset(j as isize),
            );
            j += 1;
        }
        len = norm(curr_vector, 0 as libc::c_int, n - 1 as libc::c_int);
        vecscale(
            curr_vector,
            0 as libc::c_int,
            n - 1 as libc::c_int,
            1.0f64 / len,
            curr_vector,
        );
        *evals.offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < neigs - 1 as libc::c_int {
        largest_index = i;
        largest_eval = *evals.offset(largest_index as isize);
        j = i + 1 as libc::c_int;
        while j < neigs {
            if largest_eval < *evals.offset(j as isize) {
                largest_index = j;
                largest_eval = *evals.offset(largest_index as isize);
            }
            j += 1;
        }
        if largest_index != i {
            cpvec(
                tmp_vec,
                0 as libc::c_int,
                n - 1 as libc::c_int,
                *eigs.offset(i as isize),
            );
            cpvec(
                *eigs.offset(i as isize),
                0 as libc::c_int,
                n - 1 as libc::c_int,
                *eigs.offset(largest_index as isize),
            );
            cpvec(
                *eigs.offset(largest_index as isize),
                0 as libc::c_int,
                n - 1 as libc::c_int,
                tmp_vec,
            );
            *evals.offset(largest_index as isize) = *evals.offset(i as isize);
            *evals.offset(i as isize) = largest_eval;
        }
        i += 1;
    }
    free(tmp_vec as *mut libc::c_void);
    free(last_vec as *mut libc::c_void);
}
unsafe extern "C" fn compute_avgs(
    mut Dij: *mut *mut DistType,
    mut n: libc::c_int,
    mut all_avg: *mut libc::c_float,
) -> *mut libc::c_float {
    let mut row_avg: *mut libc::c_float = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sum_row: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < n {
        sum_row = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int;
        while j < n {
            sum += *(*Dij.offset(i as isize)).offset(j as isize) as libc::c_double
                * *(*Dij.offset(i as isize)).offset(j as isize) as libc::c_double;
            sum_row += *(*Dij.offset(i as isize)).offset(j as isize) as libc::c_double
                * *(*Dij.offset(i as isize)).offset(j as isize) as libc::c_double;
            j += 1;
        }
        *row_avg.offset(i as isize) = sum_row as libc::c_float / n as libc::c_float;
        i += 1;
    }
    *all_avg = sum as libc::c_float / (n * n) as libc::c_float;
    return row_avg;
}
unsafe extern "C" fn compute_Bij(
    mut Dij: *mut *mut DistType,
    mut n: libc::c_int,
) -> *mut *mut libc::c_float {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut storage: *mut libc::c_float = gcalloc(
        (n * n) as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut Bij: *mut *mut libc::c_float = gcalloc(
        n as size_t,
        ::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong,
    ) as *mut *mut libc::c_float;
    let mut row_avg: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut all_avg: libc::c_float = 0.;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh0 = *Bij.offset(i as isize);
        *fresh0 = storage.offset((i * n) as isize);
        i += 1;
    }
    row_avg = compute_avgs(Dij, n, &mut all_avg);
    i = 0 as libc::c_int;
    while i < n {
        j = 0 as libc::c_int;
        while j <= i {
            *(*Bij.offset(i as isize)).offset(j as isize) =
                -(*(*Dij.offset(i as isize)).offset(j as isize) as libc::c_float)
                    * *(*Dij.offset(i as isize)).offset(j as isize) as libc::c_float
                    + *row_avg.offset(i as isize)
                    + *row_avg.offset(j as isize)
                    - all_avg;
            *(*Bij.offset(j as isize)).offset(i as isize) =
                *(*Bij.offset(i as isize)).offset(j as isize);
            j += 1;
        }
        i += 1;
    }
    free(row_avg as *mut libc::c_void);
    return Bij;
}
unsafe extern "C" fn CMDS_orthog(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut eigs: *mut *mut libc::c_double,
    mut tol: libc::c_double,
    mut orthog: *mut libc::c_double,
    mut Dij: *mut *mut DistType,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut Bij: *mut *mut libc::c_float = compute_Bij(Dij, n);
    let mut evals: *mut libc::c_double = gcalloc(
        dim as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut orthog_aux: *mut libc::c_double = 0 as *mut libc::c_double;
    if !orthog.is_null() {
        orthog_aux = gcalloc(
            n as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
        i = 0 as libc::c_int;
        while i < n {
            *orthog_aux.offset(i as isize) = *orthog.offset(i as isize);
            i += 1;
        }
        standardize(orthog_aux, n);
    }
    power_iteration_orthog(Bij, n, dim, eigs, evals, orthog_aux, tol);
    i = 0 as libc::c_int;
    while i < dim {
        j = 0 as libc::c_int;
        while j < n {
            *(*eigs.offset(i as isize)).offset(j as isize) *= sqrt(fabs(*evals.offset(i as isize)));
            j += 1;
        }
        i += 1;
    }
    free(*Bij.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free(Bij as *mut libc::c_void);
    free(evals as *mut libc::c_void);
    free(orthog_aux as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn IMDS_given_dim(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut given_coords: *mut libc::c_double,
    mut new_coords: *mut libc::c_double,
    mut conj_tol: libc::c_double,
) -> libc::c_int {
    let mut current_block: u64;
    let mut iterations2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut Dij: *mut *mut DistType = 0 as *mut *mut DistType;
    let mut f_storage: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut x: *mut libc::c_double = given_coords;
    let mut uniLength: libc::c_double = 0.;
    let mut orthog_aux: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut y: *mut libc::c_double = new_coords;
    let mut lap: *mut *mut libc::c_float = gcalloc(
        n as size_t,
        ::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong,
    ) as *mut *mut libc::c_float;
    let mut degree: libc::c_float = 0.;
    let mut pos_i: libc::c_double = 0.;
    let mut balance: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut b: libc::c_double = 0.;
    let mut converged: bool = false;
    Dij = compute_apsp(graph, n);
    i = 0 as libc::c_int;
    while i < n {
        j = 0 as libc::c_int;
        while j < n {
            let ref mut fresh1 = *(*Dij.offset(i as isize)).offset(j as isize);
            *fresh1 *= 256 as libc::c_int;
            j += 1;
        }
        i += 1;
    }
    if !x.is_null() {
    } else {
        __assert_fail(
            b"x!=NULL\0" as *const u8 as *const libc::c_char,
            b"smart_ini_x.c\0" as *const u8 as *const libc::c_char,
            277 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                b"int IMDS_given_dim(vtx_data *, int, double *, double *, double)\0",
            ))
            .as_ptr(),
        );
    }
    let mut sum1: libc::c_double = 0.;
    let mut sum2: libc::c_double = 0.;
    orthog_aux = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        *orthog_aux.offset(i as isize) = *x.offset(i as isize);
        i += 1;
    }
    standardize(orthog_aux, n);
    sum2 = 0 as libc::c_int as libc::c_double;
    sum1 = sum2;
    i = 1 as libc::c_int;
    while i < n {
        j = 0 as libc::c_int;
        while j < i {
            sum1 += 1.0f64 / *(*Dij.offset(i as isize)).offset(j as isize) as libc::c_double
                * fabs(*x.offset(i as isize) - *x.offset(j as isize));
            sum2 += 1.0f64
                / (*(*Dij.offset(i as isize)).offset(j as isize)
                    * *(*Dij.offset(i as isize)).offset(j as isize))
                    as libc::c_double
                * fabs(*x.offset(i as isize) - *x.offset(j as isize))
                * fabs(*x.offset(i as isize) - *x.offset(j as isize));
            j += 1;
        }
        i += 1;
    }
    uniLength = sum1 / sum2;
    i = 0 as libc::c_int;
    while i < n {
        *x.offset(i as isize) *= uniLength;
        i += 1;
    }
    CMDS_orthog(graph, n, 1 as libc::c_int, &mut y, conj_tol, x, Dij);
    f_storage = gcalloc(
        (n * n) as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh2 = *lap.offset(i as isize);
        *fresh2 = f_storage.offset((i * n) as isize);
        degree = 0 as libc::c_int as libc::c_float;
        j = 0 as libc::c_int;
        while j < n {
            if !(j == i) {
                let ref mut fresh3 = *(*lap.offset(i as isize)).offset(j as isize);
                *fresh3 = -1.0f32
                    / (*(*Dij.offset(i as isize)).offset(j as isize) as libc::c_float
                        * *(*Dij.offset(i as isize)).offset(j as isize) as libc::c_float);
                degree -= *fresh3;
            }
            j += 1;
        }
        *(*lap.offset(i as isize)).offset(i as isize) = degree;
        i += 1;
    }
    let mut diff: libc::c_double = 0.;
    i = 1 as libc::c_int;
    while i < n {
        pos_i = *x.offset(i as isize);
        j = 0 as libc::c_int;
        while j < i {
            diff = *(*Dij.offset(i as isize)).offset(j as isize) as libc::c_double
                * *(*Dij.offset(i as isize)).offset(j as isize) as libc::c_double
                - (pos_i - *x.offset(j as isize)) * (pos_i - *x.offset(j as isize));
            let ref mut fresh4 = *(*Dij.offset(j as isize)).offset(i as isize);
            *fresh4 = if diff > 0 as libc::c_int as libc::c_double {
                sqrt(diff) as DistType
            } else {
                0 as libc::c_int
            };
            *(*Dij.offset(i as isize)).offset(j as isize) = *fresh4;
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        pos_i = *y.offset(i as isize);
        *balance.offset(i as isize) = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int;
        while j < n {
            if !(j == i) {
                if pos_i >= *y.offset(j as isize) {
                    *balance.offset(i as isize) += (*(*Dij.offset(i as isize)).offset(j as isize)
                        as libc::c_float
                        * -*(*lap.offset(i as isize)).offset(j as isize))
                        as libc::c_double;
                } else {
                    *balance.offset(i as isize) -= (*(*Dij.offset(i as isize)).offset(j as isize)
                        as libc::c_float
                        * -*(*lap.offset(i as isize)).offset(j as isize))
                        as libc::c_double;
                }
            }
            j += 1;
        }
        i += 1;
    }
    converged = 0 as libc::c_int != 0;
    iterations2 = 0 as libc::c_int;
    loop {
        if !(iterations2 < 200 as libc::c_int && !converged) {
            current_block = 3634396408142324656;
            break;
        }
        if conjugate_gradient_f(lap, y, balance, n, conj_tol, n, 1 as libc::c_int != 0)
            < 0 as libc::c_int
        {
            rv = 1 as libc::c_int;
            current_block = 16699525519040101309;
            break;
        } else {
            converged = 1 as libc::c_int != 0;
            i = 0 as libc::c_int;
            while i < n {
                pos_i = *y.offset(i as isize);
                b = 0 as libc::c_int as libc::c_double;
                j = 0 as libc::c_int;
                while j < n {
                    if !(j == i) {
                        if pos_i >= *y.offset(j as isize) {
                            b += (*(*Dij.offset(i as isize)).offset(j as isize) as libc::c_float
                                * -*(*lap.offset(i as isize)).offset(j as isize))
                                as libc::c_double;
                        } else {
                            b -= (*(*Dij.offset(i as isize)).offset(j as isize) as libc::c_float
                                * -*(*lap.offset(i as isize)).offset(j as isize))
                                as libc::c_double;
                        }
                    }
                    j += 1;
                }
                if b != *balance.offset(i as isize)
                    && fabs(1 as libc::c_int as libc::c_double - b / *balance.offset(i as isize))
                        > 1e-5f64
                {
                    converged = 0 as libc::c_int != 0;
                    *balance.offset(i as isize) = b;
                }
                i += 1;
            }
            iterations2 += 1;
        }
    }
    match current_block {
        3634396408142324656 => {
            i = 0 as libc::c_int;
            while i < n {
                *x.offset(i as isize) /= uniLength;
                *y.offset(i as isize) /= uniLength;
                i += 1;
            }
        }
        _ => {}
    }
    free(*Dij.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free(Dij as *mut libc::c_void);
    free(*lap.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free(lap as *mut libc::c_void);
    free(orthog_aux as *mut libc::c_void);
    free(balance as *mut libc::c_void);
    return rv;
}
