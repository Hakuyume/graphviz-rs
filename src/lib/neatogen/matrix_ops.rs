#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabsf(_: libc::c_float) -> libc::c_float;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fmaxf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vtx_data {
    pub nedges: libc::c_int,
    pub edges: *mut libc::c_int,
    pub ewgts: *mut libc::c_float,
    pub eweights: *mut libc::c_float,
    pub edists: *mut libc::c_float,
}
pub type size_t = libc::c_ulong;
static mut p_iteration_threshold: libc::c_double = 1e-3f64;
#[no_mangle]
pub unsafe extern "C" fn power_iteration(
    mut square_mat: *mut *mut libc::c_double,
    mut n: libc::c_int,
    mut neigs: libc::c_int,
    mut eigs: *mut *mut libc::c_double,
    mut evals: *mut libc::c_double,
    mut initialize: libc::c_int,
) -> bool {
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
    let mut iteration: libc::c_int = 0 as libc::c_int;
    let mut largest_index: libc::c_int = 0;
    let mut largest_eval: libc::c_double = 0.;
    let mut Max_iterations: libc::c_int = 30 as libc::c_int * n;
    let mut tol: libc::c_double = 1 as libc::c_int as libc::c_double
        - p_iteration_threshold;
    if neigs >= n {
        neigs = n;
    }
    i = 0 as libc::c_int;
    's_53: while i < neigs {
        curr_vector = *eigs.offset(i as isize);
        loop {
            if initialize != 0 {
                j = 0 as libc::c_int;
                while j < n {
                    *curr_vector
                        .offset(
                            j as isize,
                        ) = (rand() % 100 as libc::c_int) as libc::c_double;
                    j += 1;
                }
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
            cpvec(last_vec, 0 as libc::c_int, n - 1 as libc::c_int, curr_vector);
            right_mult_with_vector_d(square_mat, n, n, curr_vector, tmp_vec);
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
            if len < 1e-10f64 || iteration > Max_iterations {
                break 's_53;
            }
            vecscale(
                curr_vector,
                0 as libc::c_int,
                n - 1 as libc::c_int,
                1.0f64 / len,
                curr_vector,
            );
            angle = dot(curr_vector, 0 as libc::c_int, n - 1 as libc::c_int, last_vec);
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
            *curr_vector
                .offset(j as isize) = (rand() % 100 as libc::c_int) as libc::c_double;
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
    return iteration <= Max_iterations;
}
#[no_mangle]
pub unsafe extern "C" fn mult_dense_mat(
    mut A: *mut *mut libc::c_double,
    mut B: *mut *mut libc::c_float,
    mut dim1: libc::c_int,
    mut dim2: libc::c_int,
    mut dim3: libc::c_int,
    mut CC: *mut *mut *mut libc::c_float,
) {
    let mut sum: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut storage: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut C: *mut *mut libc::c_float = *CC;
    if !C.is_null() {
        storage = realloc(
            *C.offset(0 as libc::c_int as isize) as *mut libc::c_void,
            ((dim1 * dim3) as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
                ),
        ) as *mut libc::c_float;
        C = realloc(
            C as *mut libc::c_void,
            (dim1 as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut *mut libc::c_double>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_float;
        *CC = C;
    } else {
        storage = malloc(
            ((dim1 * dim3) as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
                ),
        ) as *mut libc::c_float;
        C = malloc(
            (dim1 as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut *mut libc::c_double>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_float;
        *CC = C;
    }
    i = 0 as libc::c_int;
    while i < dim1 {
        let ref mut fresh0 = *C.offset(i as isize);
        *fresh0 = storage;
        storage = storage.offset(dim3 as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < dim1 {
        j = 0 as libc::c_int;
        while j < dim3 {
            sum = 0 as libc::c_int as libc::c_double;
            k = 0 as libc::c_int;
            while k < dim2 {
                sum
                    += *(*A.offset(i as isize)).offset(k as isize)
                        * *(*B.offset(k as isize)).offset(j as isize) as libc::c_double;
                k += 1;
            }
            *(*C.offset(i as isize)).offset(j as isize) = sum as libc::c_float;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mult_dense_mat_d(
    mut A: *mut *mut libc::c_double,
    mut B: *mut *mut libc::c_float,
    mut dim1: libc::c_int,
    mut dim2: libc::c_int,
    mut dim3: libc::c_int,
    mut CC: *mut *mut *mut libc::c_double,
) {
    let mut C: *mut *mut libc::c_double = *CC;
    let mut storage: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
    if !C.is_null() {
        storage = realloc(
            *C.offset(0 as libc::c_int as isize) as *mut libc::c_void,
            ((dim1 * dim3) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double;
        C = realloc(
            C as *mut libc::c_void,
            (dim1 as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_double;
        *CC = C;
    } else {
        storage = malloc(
            ((dim1 * dim3) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double;
        C = malloc(
            (dim1 as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_double;
        *CC = C;
    }
    i = 0 as libc::c_int;
    while i < dim1 {
        let ref mut fresh1 = *C.offset(i as isize);
        *fresh1 = storage;
        storage = storage.offset(dim3 as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < dim1 {
        j = 0 as libc::c_int;
        while j < dim3 {
            sum = 0 as libc::c_int as libc::c_double;
            k = 0 as libc::c_int;
            while k < dim2 {
                sum
                    += *(*A.offset(i as isize)).offset(k as isize)
                        * *(*B.offset(k as isize)).offset(j as isize) as libc::c_double;
                k += 1;
            }
            *(*C.offset(i as isize)).offset(j as isize) = sum;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mult_sparse_dense_mat_transpose(
    mut A: *mut vtx_data,
    mut B: *mut *mut libc::c_double,
    mut dim1: libc::c_int,
    mut dim2: libc::c_int,
    mut CC: *mut *mut *mut libc::c_float,
) {
    let mut storage: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
    let mut ewgts: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut edges: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nedges: libc::c_int = 0;
    let mut C: *mut *mut libc::c_float = *CC;
    if !C.is_null() {
        storage = realloc(
            *C.offset(0 as libc::c_int as isize) as *mut libc::c_void,
            ((dim1 * dim2) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<vtx_data>() as libc::c_ulong),
        ) as *mut libc::c_float;
        C = realloc(
            C as *mut libc::c_void,
            (dim1 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut vtx_data>() as libc::c_ulong),
        ) as *mut *mut libc::c_float;
        *CC = C;
    } else {
        storage = malloc(
            ((dim1 * dim2) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<vtx_data>() as libc::c_ulong),
        ) as *mut libc::c_float;
        C = malloc(
            (dim1 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut vtx_data>() as libc::c_ulong),
        ) as *mut *mut libc::c_float;
        *CC = C;
    }
    i = 0 as libc::c_int;
    while i < dim1 {
        let ref mut fresh2 = *C.offset(i as isize);
        *fresh2 = storage;
        storage = storage.offset(dim2 as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < dim1 {
        edges = (*A.offset(i as isize)).edges;
        ewgts = (*A.offset(i as isize)).ewgts;
        nedges = (*A.offset(i as isize)).nedges;
        j = 0 as libc::c_int;
        while j < dim2 {
            sum = 0 as libc::c_int as libc::c_double;
            k = 0 as libc::c_int;
            while k < nedges {
                sum
                    += *ewgts.offset(k as isize) as libc::c_double
                        * *(*B.offset(j as isize))
                            .offset(*edges.offset(k as isize) as isize);
                k += 1;
            }
            *(*C.offset(i as isize)).offset(j as isize) = sum as libc::c_float;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cpvec(
    mut copy: *mut libc::c_double,
    mut beg: libc::c_int,
    mut end: libc::c_int,
    mut vec: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    copy = copy.offset(beg as isize);
    vec = vec.offset(beg as isize);
    i = end - beg + 1 as libc::c_int;
    while i != 0 {
        let fresh3 = vec;
        vec = vec.offset(1);
        let fresh4 = copy;
        copy = copy.offset(1);
        *fresh4 = *fresh3;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dot(
    mut vec1: *mut libc::c_double,
    mut beg: libc::c_int,
    mut end: libc::c_int,
    mut vec2: *mut libc::c_double,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
    sum = 0.0f64;
    vec1 = vec1.offset(beg as isize);
    vec2 = vec2.offset(beg as isize);
    i = end - beg + 1 as libc::c_int;
    while i != 0 {
        let fresh5 = vec1;
        vec1 = vec1.offset(1);
        let fresh6 = vec2;
        vec2 = vec2.offset(1);
        sum += *fresh5 * *fresh6;
        i -= 1;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn scadd(
    mut vec1: *mut libc::c_double,
    mut beg: libc::c_int,
    mut end: libc::c_int,
    mut fac: libc::c_double,
    mut vec2: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    vec1 = vec1.offset(beg as isize);
    vec2 = vec2.offset(beg as isize);
    i = end - beg + 1 as libc::c_int;
    while i != 0 {
        let fresh7 = vec2;
        vec2 = vec2.offset(1);
        let fresh8 = vec1;
        vec1 = vec1.offset(1);
        *fresh8 += fac * *fresh7;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vecscale(
    mut vec1: *mut libc::c_double,
    mut beg: libc::c_int,
    mut end: libc::c_int,
    mut alpha: libc::c_double,
    mut vec2: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    vec1 = vec1.offset(beg as isize);
    vec2 = vec2.offset(beg as isize);
    i = end - beg + 1 as libc::c_int;
    while i != 0 {
        let fresh9 = vec2;
        vec2 = vec2.offset(1);
        let fresh10 = vec1;
        vec1 = vec1.offset(1);
        *fresh10 = alpha * *fresh9;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn norm(
    mut vec: *mut libc::c_double,
    mut beg: libc::c_int,
    mut end: libc::c_int,
) -> libc::c_double {
    return sqrt(dot(vec, beg, end, vec));
}
#[no_mangle]
pub unsafe extern "C" fn orthog1(mut n: libc::c_int, mut vec: *mut libc::c_double) {
    let mut i: libc::c_int = 0;
    let mut pntr: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut sum: libc::c_double = 0.;
    sum = 0.0f64;
    pntr = vec;
    i = n;
    while i != 0 {
        let fresh11 = pntr;
        pntr = pntr.offset(1);
        sum += *fresh11;
        i -= 1;
    }
    sum /= n as libc::c_double;
    pntr = vec;
    i = n;
    while i != 0 {
        let fresh12 = pntr;
        pntr = pntr.offset(1);
        *fresh12 -= sum;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_vec_orth1(
    mut n: libc::c_int,
    mut vec: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *vec.offset(i as isize) = (rand() % 500 as libc::c_int) as libc::c_double;
        i += 1;
    }
    orthog1(n, vec);
}
#[no_mangle]
pub unsafe extern "C" fn right_mult_with_vector(
    mut matrix: *mut vtx_data,
    mut n: libc::c_int,
    mut vector: *mut libc::c_double,
    mut result: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut res: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < n {
        res = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int;
        while j < (*matrix.offset(i as isize)).nedges {
            res
                += *((*matrix.offset(i as isize)).ewgts).offset(j as isize)
                    as libc::c_double
                    * *vector
                        .offset(
                            *((*matrix.offset(i as isize)).edges).offset(j as isize)
                                as isize,
                        );
            j += 1;
        }
        *result.offset(i as isize) = res;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn right_mult_with_vector_f(
    mut matrix: *mut *mut libc::c_float,
    mut n: libc::c_int,
    mut vector: *mut libc::c_double,
    mut result: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut res: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < n {
        res = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int;
        while j < n {
            res
                += *(*matrix.offset(i as isize)).offset(j as isize) as libc::c_double
                    * *vector.offset(j as isize);
            j += 1;
        }
        *result.offset(i as isize) = res;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vectors_subtraction(
    mut n: libc::c_int,
    mut vector1: *mut libc::c_double,
    mut vector2: *mut libc::c_double,
    mut result: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *result
            .offset(
                i as isize,
            ) = *vector1.offset(i as isize) - *vector2.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vectors_addition(
    mut n: libc::c_int,
    mut vector1: *mut libc::c_double,
    mut vector2: *mut libc::c_double,
    mut result: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *result
            .offset(
                i as isize,
            ) = *vector1.offset(i as isize) + *vector2.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vectors_scalar_mult(
    mut n: libc::c_int,
    mut vector: *mut libc::c_double,
    mut alpha: libc::c_double,
    mut result: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *result.offset(i as isize) = *vector.offset(i as isize) * alpha;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn copy_vector(
    mut n: libc::c_int,
    mut source: *mut libc::c_double,
    mut dest: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *dest.offset(i as isize) = *source.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vectors_inner_product(
    mut n: libc::c_int,
    mut vector1: *mut libc::c_double,
    mut vector2: *mut libc::c_double,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut result: libc::c_double = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        result += *vector1.offset(i as isize) * *vector2.offset(i as isize);
        i += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn max_abs(
    mut n: libc::c_int,
    mut vector: *mut libc::c_double,
) -> libc::c_double {
    let mut max_val: libc::c_double = -1e50f64;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        max_val = fmax(max_val, fabs(*vector.offset(i as isize)));
        i += 1;
    }
    return max_val;
}
#[no_mangle]
pub unsafe extern "C" fn right_mult_with_vector_transpose(
    mut matrix: *mut *mut libc::c_double,
    mut dim1: libc::c_int,
    mut dim2: libc::c_int,
    mut vector: *mut libc::c_double,
    mut result: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut res: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < dim1 {
        res = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int;
        while j < dim2 {
            res
                += *(*matrix.offset(j as isize)).offset(i as isize)
                    * *vector.offset(j as isize);
            j += 1;
        }
        *result.offset(i as isize) = res;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn right_mult_with_vector_d(
    mut matrix: *mut *mut libc::c_double,
    mut dim1: libc::c_int,
    mut dim2: libc::c_int,
    mut vector: *mut libc::c_double,
    mut result: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut res: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < dim1 {
        res = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int;
        while j < dim2 {
            res
                += *(*matrix.offset(i as isize)).offset(j as isize)
                    * *vector.offset(j as isize);
            j += 1;
        }
        *result.offset(i as isize) = res;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn orthog1f(mut n: libc::c_int, mut vec: *mut libc::c_float) {
    let mut i: libc::c_int = 0;
    let mut pntr: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut sum: libc::c_float = 0.;
    sum = 0.0f64 as libc::c_float;
    pntr = vec;
    i = n;
    while i != 0 {
        let fresh13 = pntr;
        pntr = pntr.offset(1);
        sum += *fresh13;
        i -= 1;
    }
    sum /= n as libc::c_float;
    pntr = vec;
    i = n;
    while i != 0 {
        let fresh14 = pntr;
        pntr = pntr.offset(1);
        *fresh14 -= sum;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn right_mult_with_vector_ff(
    mut packed_matrix: *mut libc::c_float,
    mut n: libc::c_int,
    mut vector: *mut libc::c_float,
    mut result: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut vector_i: libc::c_float = 0.;
    let mut res: libc::c_float = 0.;
    i = 0 as libc::c_int;
    while i < n {
        *result.offset(i as isize) = 0 as libc::c_int as libc::c_float;
        i += 1;
    }
    index = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        res = 0 as libc::c_int as libc::c_float;
        vector_i = *vector.offset(i as isize);
        let fresh15 = index;
        index = index + 1;
        res += *packed_matrix.offset(fresh15 as isize) * vector_i;
        j = i + 1 as libc::c_int;
        while j < n {
            res += *packed_matrix.offset(index as isize) * *vector.offset(j as isize);
            *result.offset(j as isize)
                += *packed_matrix.offset(index as isize) * vector_i;
            j += 1;
            index += 1;
        }
        *result.offset(i as isize) += res;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vectors_substractionf(
    mut n: libc::c_int,
    mut vector1: *mut libc::c_float,
    mut vector2: *mut libc::c_float,
    mut result: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *result
            .offset(
                i as isize,
            ) = *vector1.offset(i as isize) - *vector2.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vectors_additionf(
    mut n: libc::c_int,
    mut vector1: *mut libc::c_float,
    mut vector2: *mut libc::c_float,
    mut result: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *result
            .offset(
                i as isize,
            ) = *vector1.offset(i as isize) + *vector2.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vectors_mult_additionf(
    mut n: libc::c_int,
    mut vector1: *mut libc::c_float,
    mut alpha: libc::c_float,
    mut vector2: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *vector1
            .offset(
                i as isize,
            ) = *vector1.offset(i as isize) + alpha * *vector2.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vectors_scalar_multf(
    mut n: libc::c_int,
    mut vector: *mut libc::c_float,
    mut alpha: libc::c_float,
    mut result: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *result.offset(i as isize) = *vector.offset(i as isize) * alpha;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn copy_vectorf(
    mut n: libc::c_int,
    mut source: *mut libc::c_float,
    mut dest: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *dest.offset(i as isize) = *source.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vectors_inner_productf(
    mut n: libc::c_int,
    mut vector1: *mut libc::c_float,
    mut vector2: *mut libc::c_float,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut result: libc::c_double = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        result
            += (*vector1.offset(i as isize) * *vector2.offset(i as isize))
                as libc::c_double;
        i += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn set_vector_val(
    mut n: libc::c_int,
    mut val: libc::c_double,
    mut result: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *result.offset(i as isize) = val;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_vector_valf(
    mut n: libc::c_int,
    mut val: libc::c_float,
    mut result: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *result.offset(i as isize) = val;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn max_absf(
    mut n: libc::c_int,
    mut vector: *mut libc::c_float,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut max_val: libc::c_float = -1e30f32;
    i = 0 as libc::c_int;
    while i < n {
        max_val = fmaxf(max_val, fabsf(*vector.offset(i as isize)));
        i += 1;
    }
    return max_val as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn square_vec(mut n: libc::c_int, mut vec: *mut libc::c_float) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *vec.offset(i as isize) *= *vec.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn invert_vec(mut n: libc::c_int, mut vec: *mut libc::c_float) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        if *vec.offset(i as isize) as libc::c_double != 0.0f64 {
            *vec.offset(i as isize) = 1.0f32 / *vec.offset(i as isize);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqrt_vec(mut n: libc::c_int, mut vec: *mut libc::c_float) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *vec.offset(i as isize) = sqrtf(*vec.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqrt_vecf(
    mut n: libc::c_int,
    mut source: *mut libc::c_float,
    mut target: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        if *source.offset(i as isize) as libc::c_double >= 0.0f64 {
            *target.offset(i as isize) = sqrtf(*source.offset(i as isize));
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn invert_sqrt_vec(
    mut n: libc::c_int,
    mut vec: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        if *vec.offset(i as isize) as libc::c_double > 0.0f64 {
            *vec.offset(i as isize) = 1.0f32 / sqrtf(*vec.offset(i as isize));
        }
        i += 1;
    }
}
