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
    fn mult_dense_mat_d(
        _: *mut *mut libc::c_double,
        _: *mut *mut libc::c_float,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        CC: *mut *mut *mut libc::c_double,
    );
    fn mult_sparse_dense_mat_transpose(
        _: *mut vtx_data,
        _: *mut *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut *mut *mut libc::c_float,
    );
    fn power_iteration(
        _: *mut *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut *mut libc::c_double,
        _: *mut libc::c_double,
        _: libc::c_int,
    ) -> bool;
    fn free(_: *mut libc::c_void);
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn closest_pairs2graph(
        _: *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut *mut vtx_data,
    );
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
pub type DistType = libc::c_int;
pub type size_t = libc::c_ulong;
static mut num_pairs: libc::c_int = 4 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn PCA_alloc(
    mut coords: *mut *mut DistType,
    mut dim: libc::c_int,
    mut n: libc::c_int,
    mut new_coords: *mut *mut libc::c_double,
    mut new_dim: libc::c_int,
) {
    let mut DD: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut sum: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut eigs: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut evals: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut storage_ptr: *mut libc::c_double = 0 as *mut libc::c_double;
    eigs = gcalloc(
        new_dim as size_t,
        ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
    ) as *mut *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < new_dim {
        let ref mut fresh0 = *eigs.offset(i as isize);
        *fresh0 = gcalloc(
            dim as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
        i += 1;
    }
    evals = gcalloc(
        new_dim as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    DD = gcalloc(
        dim as size_t,
        ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
    ) as *mut *mut libc::c_double;
    storage_ptr = gcalloc(
        (dim * dim) as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < dim {
        let ref mut fresh1 = *DD.offset(i as isize);
        *fresh1 = storage_ptr;
        storage_ptr = storage_ptr.offset(dim as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < dim {
        j = 0 as libc::c_int;
        while j <= i {
            sum = 0 as libc::c_int as libc::c_double;
            k = 0 as libc::c_int;
            while k < n {
                sum += (*(*coords.offset(i as isize)).offset(k as isize)
                    * *(*coords.offset(j as isize)).offset(k as isize))
                    as libc::c_double;
                k += 1;
            }
            let ref mut fresh2 = *(*DD.offset(j as isize)).offset(i as isize);
            *fresh2 = sum;
            *(*DD.offset(i as isize)).offset(j as isize) = *fresh2;
            j += 1;
        }
        i += 1;
    }
    power_iteration(
        DD,
        dim,
        new_dim,
        eigs,
        evals,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    j = 0 as libc::c_int;
    while j < new_dim {
        i = 0 as libc::c_int;
        while i < n {
            sum = 0 as libc::c_int as libc::c_double;
            k = 0 as libc::c_int;
            while k < dim {
                sum += *(*coords.offset(k as isize)).offset(i as isize) as libc::c_double
                    * *(*eigs.offset(j as isize)).offset(k as isize);
                k += 1;
            }
            *(*new_coords.offset(j as isize)).offset(i as isize) = sum;
            i += 1;
        }
        j += 1;
    }
    i = 0 as libc::c_int;
    while i < new_dim {
        free(*eigs.offset(i as isize) as *mut libc::c_void);
        i += 1;
    }
    free(eigs as *mut libc::c_void);
    free(evals as *mut libc::c_void);
    free(*DD.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free(DD as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn iterativePCA_1D(
    mut coords: *mut *mut libc::c_double,
    mut dim: libc::c_int,
    mut n: libc::c_int,
    mut new_direction: *mut libc::c_double,
) -> bool {
    let mut laplacian: *mut vtx_data = 0 as *mut vtx_data;
    let mut mat1: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut mat: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut eval: libc::c_double = 0.;
    closest_pairs2graph(
        *coords.offset(0 as libc::c_int as isize),
        n,
        num_pairs * n,
        &mut laplacian,
    );
    mult_sparse_dense_mat_transpose(laplacian, coords, n, dim, &mut mat1);
    mult_dense_mat_d(coords, mat1, dim, n, dim, &mut mat);
    free(*mat1.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free(mat1 as *mut libc::c_void);
    return power_iteration(
        mat,
        dim,
        1 as libc::c_int,
        &mut new_direction,
        &mut eval,
        (0 as libc::c_int == 0) as libc::c_int,
    );
}
