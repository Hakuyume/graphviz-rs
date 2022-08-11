#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn free(_: *mut libc::c_void);
    fn SparseMatrix_symmetrize(
        A: SparseMatrix,
        pattern_symmetric_only: bool,
    ) -> SparseMatrix;
    fn SparseMatrix_coordinate_form_add_entry(
        A: SparseMatrix,
        irn: libc::c_int,
        jcn: libc::c_int,
        val: *mut libc::c_void,
    ) -> SparseMatrix;
    fn SparseMatrix_delete(A: SparseMatrix);
    fn SparseMatrix_from_coordinate_format(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_new(
        m: libc::c_int,
        n: libc::c_int,
        nz: libc::c_int,
        type_0: libc::c_int,
        format: libc::c_int,
    ) -> SparseMatrix;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn freeGraph(graph: *mut v_data);
    fn delaunay_tri(
        x: *mut libc::c_double,
        y: *mut libc::c_double,
        n: libc::c_int,
        nedges: *mut libc::c_int,
    ) -> *mut libc::c_int;
    fn UG_graph(
        x: *mut libc::c_double,
        y: *mut libc::c_double,
        n: libc::c_int,
        accurate_computation: libc::c_int,
    ) -> *mut v_data;
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const FORMAT_COORD: C2RustUnnamed = 2;
pub const FORMAT_CSR: C2RustUnnamed = 1;
pub const FORMAT_CSC: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SparseMatrix_struct {
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nz: libc::c_int,
    pub nzmax: libc::c_int,
    pub type_0: libc::c_int,
    pub ia: *mut libc::c_int,
    pub ja: *mut libc::c_int,
    pub a: *mut libc::c_void,
    pub format: libc::c_int,
    pub property: libc::c_int,
    pub size: libc::c_int,
}
pub type SparseMatrix = *mut SparseMatrix_struct;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MATRIX_TYPE_UNKNOWN: C2RustUnnamed_0 = 16;
pub const MATRIX_TYPE_PATTERN: C2RustUnnamed_0 = 8;
pub const MATRIX_TYPE_INTEGER: C2RustUnnamed_0 = 4;
pub const MATRIX_TYPE_COMPLEX: C2RustUnnamed_0 = 2;
pub const MATRIX_TYPE_REAL: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v_data {
    pub nedges: libc::c_int,
    pub edges: *mut libc::c_int,
    pub ewgts: *mut libc::c_float,
}
#[no_mangle]
pub unsafe extern "C" fn call_tri(
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
) -> SparseMatrix {
    let mut one: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut A: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut edgelist: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut xv: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut yv: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut numberofedges: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *xv.offset(i as isize) = *x.offset((i * 2 as libc::c_int) as isize);
        *yv
            .offset(
                i as isize,
            ) = *x.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize);
        i += 1;
    }
    if n > 2 as libc::c_int {
        edgelist = delaunay_tri(xv, yv, n, &mut numberofedges);
    }
    A = SparseMatrix_new(
        n,
        n,
        1 as libc::c_int,
        MATRIX_TYPE_REAL as libc::c_int,
        FORMAT_COORD as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < numberofedges {
        ii = *edgelist.offset((i * 2 as libc::c_int) as isize);
        jj = *edgelist.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize);
        SparseMatrix_coordinate_form_add_entry(
            A,
            ii,
            jj,
            &mut one as *mut libc::c_double as *mut libc::c_void,
        );
        i += 1;
    }
    if n == 2 as libc::c_int {
        ii = 0 as libc::c_int;
        jj = 1 as libc::c_int;
        SparseMatrix_coordinate_form_add_entry(
            A,
            ii,
            jj,
            &mut one as *mut libc::c_double as *mut libc::c_void,
        );
    }
    i = 0 as libc::c_int;
    while i < n {
        SparseMatrix_coordinate_form_add_entry(
            A,
            i,
            i,
            &mut one as *mut libc::c_double as *mut libc::c_void,
        );
        i += 1;
    }
    B = SparseMatrix_from_coordinate_format(A);
    SparseMatrix_delete(A);
    A = SparseMatrix_symmetrize(B, 0 as libc::c_int != 0);
    SparseMatrix_delete(B);
    B = A;
    free(edgelist as *mut libc::c_void);
    free(xv as *mut libc::c_void);
    free(yv as *mut libc::c_void);
    return B;
}
#[no_mangle]
pub unsafe extern "C" fn call_tri2(
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut xx: *mut libc::c_double,
) -> SparseMatrix {
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut delaunay: *mut v_data = 0 as *mut v_data;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut A: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut one: libc::c_double = 1 as libc::c_int as libc::c_double;
    x = gcalloc(n as size_t, ::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as *mut libc::c_double;
    y = gcalloc(n as size_t, ::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        *x.offset(i as isize) = *xx.offset((dim * i) as isize);
        *y.offset(i as isize) = *xx.offset((dim * i + 1 as libc::c_int) as isize);
        i += 1;
    }
    delaunay = UG_graph(x, y, n, 0 as libc::c_int);
    A = SparseMatrix_new(
        n,
        n,
        1 as libc::c_int,
        MATRIX_TYPE_REAL as libc::c_int,
        FORMAT_COORD as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < n {
        j = 1 as libc::c_int;
        while j < (*delaunay.offset(i as isize)).nedges {
            SparseMatrix_coordinate_form_add_entry(
                A,
                i,
                *((*delaunay.offset(i as isize)).edges).offset(j as isize),
                &mut one as *mut libc::c_double as *mut libc::c_void,
            );
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        SparseMatrix_coordinate_form_add_entry(
            A,
            i,
            i,
            &mut one as *mut libc::c_double as *mut libc::c_void,
        );
        i += 1;
    }
    B = SparseMatrix_from_coordinate_format(A);
    B = SparseMatrix_symmetrize(B, 0 as libc::c_int != 0);
    SparseMatrix_delete(A);
    free(x as *mut libc::c_void);
    free(y as *mut libc::c_void);
    freeGraph(delaunay);
    return B;
}
