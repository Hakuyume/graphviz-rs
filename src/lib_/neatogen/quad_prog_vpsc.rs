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
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type Variable;
    pub type Constraint;
    pub type VPSC;
    fn fabs(_: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn compute_hierarchy(
        _: *mut vtx_data,
        _: libc::c_int,
        _: libc::c_double,
        _: libc::c_double,
        _: *mut libc::c_double,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn newVariable(
        id: libc::c_int,
        desiredPos: libc::c_double,
        weight: libc::c_double,
    ) -> *mut Variable;
    fn setVariableDesiredPos(_: *mut Variable, desiredPos: libc::c_double);
    fn getVariablePos(_: *mut Variable) -> libc::c_double;
    fn newConstraint(
        left: *mut Variable,
        right: *mut Variable,
        gap: libc::c_double,
    ) -> *mut Constraint;
    fn deleteVPSC(_: *mut VPSC);
    fn deleteConstraint(_: *mut Constraint);
    fn deleteVariable(_: *mut Variable);
    fn newConstraints(m: libc::c_int) -> *mut *mut Constraint;
    fn deleteConstraints(m: libc::c_int, _: *mut *mut Constraint);
    fn remapInConstraints(u: *mut Variable, v: *mut Variable, dgap: libc::c_double);
    fn remapOutConstraints(u: *mut Variable, v: *mut Variable, dgap: libc::c_double);
    fn genXConstraints(
        n: libc::c_int,
        _: *mut boxf,
        vs: *mut *mut Variable,
        cs: *mut *mut *mut Constraint,
        transitiveClosure: bool,
    ) -> libc::c_int;
    fn genYConstraints(
        n: libc::c_int,
        _: *mut boxf,
        vs: *mut *mut Variable,
        cs: *mut *mut *mut Constraint,
    ) -> libc::c_int;
    fn satisfyVPSC(_: *mut VPSC);
    fn solveVPSC(_: *mut VPSC);
    fn newIncVPSC(
        n: libc::c_int,
        vs: *mut *mut Variable,
        m: libc::c_int,
        cs: *mut *mut Constraint,
    ) -> *mut VPSC;
    fn unpackMatrix(packedMat: *mut libc::c_float, n: libc::c_int) -> *mut *mut libc::c_float;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pointf_s {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type pointf = pointf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct boxf {
    pub LL: pointf,
    pub UR: pointf,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cluster_data {
    pub nvars: libc::c_int,
    pub nclusters: libc::c_int,
    pub clustersizes: *mut libc::c_int,
    pub clusters: *mut *mut libc::c_int,
    pub ntoplevel: libc::c_int,
    pub toplevel: *mut libc::c_int,
    pub bb: *mut boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipsep_options {
    pub diredges: libc::c_int,
    pub edge_gap: libc::c_double,
    pub noverlap: libc::c_int,
    pub gap: pointf,
    pub nsize: *mut pointf,
    pub clusters: *mut cluster_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMajEnvVPSC {
    pub A: *mut *mut libc::c_float,
    pub packedMat: *mut libc::c_float,
    pub nv: libc::c_int,
    pub nldv: libc::c_int,
    pub ndv: libc::c_int,
    pub vs: *mut *mut Variable,
    pub m: libc::c_int,
    pub gm: libc::c_int,
    pub cs: *mut *mut Constraint,
    pub gcs: *mut *mut Constraint,
    pub vpsc: *mut VPSC,
    pub fArray1: *mut libc::c_float,
    pub fArray2: *mut libc::c_float,
    pub fArray3: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DigColaLevel {
    pub nodes: *mut libc::c_int,
    pub num_nodes: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn constrained_majorization_vpsc(
    mut e: *mut CMajEnvVPSC,
    mut b: *mut libc::c_float,
    mut place: *mut libc::c_float,
    mut max_iterations: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut counter: libc::c_int = 0;
    let mut g: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut old_place: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut d: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut n: libc::c_int = (*e).nv + (*e).nldv;
    let mut converged: bool = 0 as libc::c_int != 0;
    if max_iterations == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    g = (*e).fArray1;
    old_place = (*e).fArray2;
    d = (*e).fArray3;
    if (*e).m > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < n {
            setVariableDesiredPos(
                *((*e).vs).offset(i as isize),
                *place.offset(i as isize) as libc::c_double,
            );
            i += 1;
        }
        satisfyVPSC((*e).vpsc);
        i = 0 as libc::c_int;
        while i < n {
            *place.offset(i as isize) =
                getVariablePos(*((*e).vs).offset(i as isize)) as libc::c_float;
            i += 1;
        }
    }
    counter = 0 as libc::c_int;
    while counter < max_iterations && !converged {
        let mut test: libc::c_float = 0 as libc::c_int as libc::c_float;
        let mut alpha: libc::c_float = 0.;
        let mut beta: libc::c_float = 0.;
        let mut numerator: libc::c_float = 0 as libc::c_int as libc::c_float;
        let mut denominator: libc::c_float = 0 as libc::c_int as libc::c_float;
        let mut r: libc::c_float = 0.;
        converged = 1 as libc::c_int != 0;
        i = 0 as libc::c_int;
        while i < n {
            *old_place.offset(i as isize) = *place.offset(i as isize);
            *g.offset(i as isize) = 2 as libc::c_int as libc::c_float * *b.offset(i as isize);
            j = 0 as libc::c_int;
            while j < n {
                *g.offset(i as isize) -= 2 as libc::c_int as libc::c_float
                    * *(*((*e).A).offset(i as isize)).offset(j as isize)
                    * *place.offset(j as isize);
                j += 1;
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < n {
            numerator += *g.offset(i as isize) * *g.offset(i as isize);
            r = 0 as libc::c_int as libc::c_float;
            j = 0 as libc::c_int;
            while j < n {
                r += 2 as libc::c_int as libc::c_float
                    * *(*((*e).A).offset(i as isize)).offset(j as isize)
                    * *g.offset(j as isize);
                j += 1;
            }
            denominator -= r * *g.offset(i as isize);
            i += 1;
        }
        if denominator != 0 as libc::c_int as libc::c_float {
            alpha = numerator / denominator;
        } else {
            alpha = 1.0f64 as libc::c_float;
        }
        i = 0 as libc::c_int;
        while i < n {
            *place.offset(i as isize) -= alpha * *g.offset(i as isize);
            i += 1;
        }
        if (*e).m > 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < n {
                setVariableDesiredPos(
                    *((*e).vs).offset(i as isize),
                    *place.offset(i as isize) as libc::c_double,
                );
                i += 1;
            }
            satisfyVPSC((*e).vpsc);
            i = 0 as libc::c_int;
            while i < n {
                *place.offset(i as isize) =
                    getVariablePos(*((*e).vs).offset(i as isize)) as libc::c_float;
                i += 1;
            }
        }
        i = 0 as libc::c_int;
        while i < n {
            *d.offset(i as isize) = *place.offset(i as isize) - *old_place.offset(i as isize);
            i += 1;
        }
        numerator = 0 as libc::c_int as libc::c_float;
        denominator = 0 as libc::c_int as libc::c_float;
        i = 0 as libc::c_int;
        while i < n {
            numerator += *g.offset(i as isize) * *d.offset(i as isize);
            r = 0 as libc::c_int as libc::c_float;
            j = 0 as libc::c_int;
            while j < n {
                r += 2 as libc::c_int as libc::c_float
                    * *(*((*e).A).offset(i as isize)).offset(j as isize)
                    * *d.offset(j as isize);
                j += 1;
            }
            denominator += r * *d.offset(i as isize);
            i += 1;
        }
        if denominator as libc::c_double != 0.0f64 {
            beta = numerator / denominator;
        } else {
            beta = 1.0f64 as libc::c_float;
        }
        i = 0 as libc::c_int;
        while i < n {
            if beta > 0 as libc::c_int as libc::c_float && (beta as libc::c_double) < 1.0f64 {
                *place.offset(i as isize) =
                    *old_place.offset(i as isize) + beta * *d.offset(i as isize);
            }
            test = (test as libc::c_double
                + fabs(
                    (*place.offset(i as isize) - *old_place.offset(i as isize)) as libc::c_double,
                )) as libc::c_float;
            i += 1;
        }
        if test as libc::c_double > 1e-4f64 {
            converged = 0 as libc::c_int != 0;
        }
        counter += 1;
    }
    return counter;
}
#[no_mangle]
pub unsafe extern "C" fn initCMajVPSC(
    mut n: libc::c_int,
    mut packedMat: *mut libc::c_float,
    mut graph: *mut vtx_data,
    mut opt: *mut ipsep_options,
    mut diredges: libc::c_int,
) -> *mut CMajEnvVPSC {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nConCs: libc::c_int = 0;
    let mut e: *mut CMajEnvVPSC =
        gmalloc(::std::mem::size_of::<CMajEnvVPSC>() as libc::c_ulong) as *mut CMajEnvVPSC;
    let ref mut fresh0 = (*e).A;
    *fresh0 = 0 as *mut *mut libc::c_float;
    let ref mut fresh1 = (*e).packedMat;
    *fresh1 = packedMat;
    (*e).nldv = 2 as libc::c_int * (*(*opt).clusters).nclusters;
    (*e).nv = n - (*e).nldv;
    (*e).ndv = 0 as libc::c_int;
    let ref mut fresh2 = (*e).gcs;
    *fresh2 = 0 as *mut *mut Constraint;
    let ref mut fresh3 = (*e).vs;
    *fresh3 = gcalloc(
        n as size_t,
        ::std::mem::size_of::<*mut Variable>() as libc::c_ulong,
    ) as *mut *mut Variable;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh4 = *((*e).vs).offset(i as isize);
        *fresh4 = newVariable(i, 1.0f64, 1.0f64);
        i += 1;
    }
    (*e).gm = 0 as libc::c_int;
    if diredges == 1 as libc::c_int {
        if Verbose != 0 {
            fprintf(
                stderr,
                b"  generate edge constraints...\n\0" as *const u8 as *const libc::c_char,
            );
        }
        i = 0 as libc::c_int;
        while i < (*e).nv {
            j = 1 as libc::c_int;
            while j < (*graph.offset(i as isize)).nedges {
                if *((*graph.offset(i as isize)).edists).offset(j as isize) as libc::c_double
                    > 0.01f64
                {
                    let ref mut fresh5 = (*e).gm;
                    *fresh5 += 1;
                }
                j += 1;
            }
            i += 1;
        }
        let ref mut fresh6 = (*e).gcs;
        *fresh6 = newConstraints((*e).gm);
        (*e).gm = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*e).nv {
            j = 1 as libc::c_int;
            while j < (*graph.offset(i as isize)).nedges {
                let mut u: libc::c_int = i;
                let mut v: libc::c_int = *((*graph.offset(i as isize)).edges).offset(j as isize);
                if *((*graph.offset(i as isize)).edists).offset(j as isize)
                    > 0 as libc::c_int as libc::c_float
                {
                    let ref mut fresh7 = (*e).gm;
                    let fresh8 = *fresh7;
                    *fresh7 = *fresh7 + 1;
                    let ref mut fresh9 = *((*e).gcs).offset(fresh8 as isize);
                    *fresh9 = newConstraint(
                        *((*e).vs).offset(u as isize),
                        *((*e).vs).offset(v as isize),
                        (*opt).edge_gap,
                    );
                }
                j += 1;
            }
            i += 1;
        }
    } else if diredges == 2 as libc::c_int {
        let mut ordering: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut ls: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut cvar: libc::c_int = 0;
        let mut halfgap: libc::c_double = 0.;
        let mut levels: *mut DigColaLevel = 0 as *mut DigColaLevel;
        let mut vs: *mut *mut Variable = (*e).vs;
        if compute_hierarchy(
            graph,
            (*e).nv,
            1e-2f64,
            1e-1f64,
            0 as *mut libc::c_double,
            &mut ordering,
            &mut ls,
            &mut (*e).ndv,
        ) != 0
        {
            return 0 as *mut CMajEnvVPSC;
        }
        levels = assign_digcola_levels(ordering, (*e).nv, ls, (*e).ndv);
        if Verbose != 0 {
            fprintf(
                stderr,
                b"Found %d DiG-CoLa boundaries\n\0" as *const u8 as *const libc::c_char,
                (*e).ndv,
            );
        }
        (*e).gm = get_num_digcola_constraints(levels, (*e).ndv + 1 as libc::c_int) + (*e).ndv
            - 1 as libc::c_int;
        let ref mut fresh10 = (*e).gcs;
        *fresh10 = newConstraints((*e).gm);
        (*e).gm = 0 as libc::c_int;
        let ref mut fresh11 = (*e).vs;
        *fresh11 = gcalloc(
            (n + (*e).ndv) as size_t,
            ::std::mem::size_of::<*mut Variable>() as libc::c_ulong,
        ) as *mut *mut Variable;
        i = 0 as libc::c_int;
        while i < n {
            let ref mut fresh12 = *((*e).vs).offset(i as isize);
            *fresh12 = *vs.offset(i as isize);
            i += 1;
        }
        free(vs as *mut libc::c_void);
        i = 0 as libc::c_int;
        while i < (*e).ndv {
            cvar = n + i;
            let ref mut fresh13 = *((*e).vs).offset(cvar as isize);
            *fresh13 = newVariable(cvar, 1.0f64, 0.000001f64);
            i += 1;
        }
        halfgap = (*opt).edge_gap;
        i = 0 as libc::c_int;
        while i < (*e).ndv {
            cvar = n + i;
            j = 0 as libc::c_int;
            while j < (*levels.offset(i as isize)).num_nodes {
                let ref mut fresh14 = (*e).gm;
                let fresh15 = *fresh14;
                *fresh14 = *fresh14 + 1;
                let ref mut fresh16 = *((*e).gcs).offset(fresh15 as isize);
                *fresh16 = newConstraint(
                    *((*e).vs)
                        .offset(*((*levels.offset(i as isize)).nodes).offset(j as isize) as isize),
                    *((*e).vs).offset(cvar as isize),
                    halfgap,
                );
                j += 1;
            }
            j = 0 as libc::c_int;
            while j < (*levels.offset((i + 1 as libc::c_int) as isize)).num_nodes {
                let ref mut fresh17 = (*e).gm;
                let fresh18 = *fresh17;
                *fresh17 = *fresh17 + 1;
                let ref mut fresh19 = *((*e).gcs).offset(fresh18 as isize);
                *fresh19 = newConstraint(
                    *((*e).vs).offset(cvar as isize),
                    *((*e).vs).offset(
                        *((*levels.offset((i + 1 as libc::c_int) as isize)).nodes)
                            .offset(j as isize) as isize,
                    ),
                    halfgap,
                );
                j += 1;
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < (*e).ndv - 1 as libc::c_int {
            let ref mut fresh20 = (*e).gm;
            let fresh21 = *fresh20;
            *fresh20 = *fresh20 + 1;
            let ref mut fresh22 = *((*e).gcs).offset(fresh21 as isize);
            *fresh22 = newConstraint(
                *((*e).vs).offset((n + i) as isize),
                *((*e).vs).offset((n + i + 1 as libc::c_int) as isize),
                0 as libc::c_int as libc::c_double,
            );
            i += 1;
        }
    }
    if (*(*opt).clusters).nclusters > 0 as libc::c_int {
        let mut ecs: *mut *mut Constraint = (*e).gcs;
        nConCs = 2 as libc::c_int * (*(*opt).clusters).nvars;
        let ref mut fresh23 = (*e).gcs;
        *fresh23 = newConstraints((*e).gm + nConCs);
        i = 0 as libc::c_int;
        while i < (*e).gm {
            let ref mut fresh24 = *((*e).gcs).offset(i as isize);
            *fresh24 = *ecs.offset(i as isize);
            i += 1;
        }
        if !ecs.is_null() {
            deleteConstraints(0 as libc::c_int, ecs);
        }
        i = 0 as libc::c_int;
        while i < (*(*opt).clusters).nclusters {
            j = 0 as libc::c_int;
            while j < *((*(*opt).clusters).clustersizes).offset(i as isize) {
                let mut v_0: *mut Variable = *((*e).vs).offset(
                    *(*((*(*opt).clusters).clusters).offset(i as isize)).offset(j as isize)
                        as isize,
                );
                let mut cl: *mut Variable =
                    *((*e).vs).offset(((*e).nv + 2 as libc::c_int * i) as isize);
                let mut cr: *mut Variable =
                    *((*e).vs).offset(((*e).nv + 2 as libc::c_int * i + 1 as libc::c_int) as isize);
                let ref mut fresh25 = (*e).gm;
                let fresh26 = *fresh25;
                *fresh25 = *fresh25 + 1;
                let ref mut fresh27 = *((*e).gcs).offset(fresh26 as isize);
                *fresh27 = newConstraint(cl, v_0, 0 as libc::c_int as libc::c_double);
                let ref mut fresh28 = (*e).gm;
                let fresh29 = *fresh28;
                *fresh28 = *fresh28 + 1;
                let ref mut fresh30 = *((*e).gcs).offset(fresh29 as isize);
                *fresh30 = newConstraint(v_0, cr, 0 as libc::c_int as libc::c_double);
                j += 1;
            }
            i += 1;
        }
    }
    (*e).m = 0 as libc::c_int;
    let ref mut fresh31 = (*e).cs;
    *fresh31 = 0 as *mut *mut Constraint;
    if (*e).gm > 0 as libc::c_int {
        let ref mut fresh32 = (*e).vpsc;
        *fresh32 = newIncVPSC(n + (*e).ndv, (*e).vs, (*e).gm, (*e).gcs);
        (*e).m = (*e).gm;
        let ref mut fresh33 = (*e).cs;
        *fresh33 = (*e).gcs;
    }
    if !packedMat.is_null() {
        let ref mut fresh34 = (*e).A;
        *fresh34 = unpackMatrix(packedMat, n);
    }
    let ref mut fresh35 = (*e).fArray1;
    *fresh35 = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let ref mut fresh36 = (*e).fArray2;
    *fresh36 = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let ref mut fresh37 = (*e).fArray3;
    *fresh37 = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    if Verbose != 0 {
        fprintf(
            stderr,
            b"  initCMajVPSC done: %d global constraints generated.\n\0" as *const u8
                as *const libc::c_char,
            (*e).m,
        );
    }
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn deleteCMajEnvVPSC(mut e: *mut CMajEnvVPSC) {
    let mut i: libc::c_int = 0;
    if !((*e).A).is_null() {
        free(*((*e).A).offset(0 as libc::c_int as isize) as *mut libc::c_void);
        free((*e).A as *mut libc::c_void);
    }
    if (*e).m > 0 as libc::c_int {
        deleteVPSC((*e).vpsc);
        if (*e).cs != (*e).gcs && !((*e).gcs).is_null() {
            deleteConstraints(0 as libc::c_int, (*e).gcs);
        }
        deleteConstraints((*e).m, (*e).cs);
        i = 0 as libc::c_int;
        while i < (*e).nv + (*e).nldv + (*e).ndv {
            deleteVariable(*((*e).vs).offset(i as isize));
            i += 1;
        }
        free((*e).vs as *mut libc::c_void);
    }
    free((*e).fArray1 as *mut libc::c_void);
    free((*e).fArray2 as *mut libc::c_void);
    free((*e).fArray3 as *mut libc::c_void);
    free(e as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn generateNonoverlapConstraints(
    mut e: *mut CMajEnvVPSC,
    mut nsizeScale: libc::c_float,
    mut coords: *mut *mut libc::c_float,
    mut k: libc::c_int,
    mut transitiveClosure: bool,
    mut opt: *mut ipsep_options,
) {
    let mut csol: *mut *mut Constraint = 0 as *mut *mut Constraint;
    let mut csolptr: *mut *mut Constraint = 0 as *mut *mut Constraint;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut mol: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = (*e).nv + (*e).nldv;
    let mut bb: *mut boxf =
        gcalloc(n as size_t, ::std::mem::size_of::<boxf>() as libc::c_ulong) as *mut boxf;
    let mut genclusters: bool = (*(*opt).clusters).nclusters > 0 as libc::c_int;
    if genclusters {
        n -= 2 as libc::c_int * (*(*opt).clusters).nclusters;
    }
    if k == 0 as libc::c_int {
        nsizeScale *= 1.0001f32;
    }
    i = 0 as libc::c_int;
    while i < n {
        (*bb.offset(i as isize)).LL.x = *(*coords.offset(0 as libc::c_int as isize))
            .offset(i as isize) as libc::c_double
            - nsizeScale as libc::c_double * (*((*opt).nsize).offset(i as isize)).x / 2.0f64
            - (*opt).gap.x / 2.0f64;
        (*bb.offset(i as isize)).UR.x = *(*coords.offset(0 as libc::c_int as isize))
            .offset(i as isize) as libc::c_double
            + nsizeScale as libc::c_double * (*((*opt).nsize).offset(i as isize)).x / 2.0f64
            + (*opt).gap.x / 2.0f64;
        (*bb.offset(i as isize)).LL.y = *(*coords.offset(1 as libc::c_int as isize))
            .offset(i as isize) as libc::c_double
            - nsizeScale as libc::c_double * (*((*opt).nsize).offset(i as isize)).y / 2.0f64
            - (*opt).gap.y / 2.0f64;
        (*bb.offset(i as isize)).UR.y = *(*coords.offset(1 as libc::c_int as isize))
            .offset(i as isize) as libc::c_double
            + nsizeScale as libc::c_double * (*((*opt).nsize).offset(i as isize)).y / 2.0f64
            + (*opt).gap.y / 2.0f64;
        i += 1;
    }
    if genclusters {
        let mut cscl: *mut *mut *mut Constraint = gcalloc(
            ((*(*opt).clusters).nclusters + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut *mut Constraint>() as libc::c_ulong,
        ) as *mut *mut *mut Constraint;
        let mut cm: *mut libc::c_int = gcalloc(
            ((*(*opt).clusters).nclusters + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        i = 0 as libc::c_int;
        while i < (*(*opt).clusters).nclusters {
            let mut cn: libc::c_int = *((*(*opt).clusters).clustersizes).offset(i as isize);
            let mut cvs: *mut *mut Variable = gcalloc(
                (cn + 2 as libc::c_int) as size_t,
                ::std::mem::size_of::<*mut Variable>() as libc::c_ulong,
            ) as *mut *mut Variable;
            let mut cbb: *mut boxf = gcalloc(
                (cn + 2 as libc::c_int) as size_t,
                ::std::mem::size_of::<boxf>() as libc::c_ulong,
            ) as *mut boxf;
            let mut container: boxf = boxf {
                LL: pointf { x: 0., y: 0. },
                UR: pointf { x: 0., y: 0. },
            };
            container.LL.y = 1.7976931348623157e+308f64;
            container.LL.x = container.LL.y;
            container.UR.y = -1.7976931348623157e+308f64;
            container.UR.x = container.UR.y;
            j = 0 as libc::c_int;
            while j < cn {
                let mut iv: libc::c_int =
                    *(*((*(*opt).clusters).clusters).offset(i as isize)).offset(j as isize);
                let ref mut fresh38 = *cvs.offset(j as isize);
                *fresh38 = *((*e).vs).offset(iv as isize);
                (*cbb.offset(j as isize)).LL.x = (*bb.offset(iv as isize)).LL.x;
                (*cbb.offset(j as isize)).LL.y = (*bb.offset(iv as isize)).LL.y;
                (*cbb.offset(j as isize)).UR.x = (*bb.offset(iv as isize)).UR.x;
                (*cbb.offset(j as isize)).UR.y = (*bb.offset(iv as isize)).UR.y;
                container.LL.x = (if container.LL.x < (*bb.offset(iv as isize)).LL.x {
                    container.LL.x
                } else {
                    (*bb.offset(iv as isize)).LL.x
                });
                container.LL.y = (if container.LL.y < (*bb.offset(iv as isize)).LL.y {
                    container.LL.y
                } else {
                    (*bb.offset(iv as isize)).LL.y
                });
                container.UR.x = (if container.UR.x > (*bb.offset(iv as isize)).UR.x {
                    container.UR.x
                } else {
                    (*bb.offset(iv as isize)).UR.x
                });
                container.UR.y = (if container.UR.y > (*bb.offset(iv as isize)).UR.y {
                    container.UR.y
                } else {
                    (*bb.offset(iv as isize)).UR.y
                });
                j += 1;
            }
            (*((*(*opt).clusters).bb).offset(i as isize)).LL.x = container.LL.x;
            (*((*(*opt).clusters).bb).offset(i as isize)).LL.y = container.LL.y;
            (*((*(*opt).clusters).bb).offset(i as isize)).UR.x = container.UR.x;
            (*((*(*opt).clusters).bb).offset(i as isize)).UR.y = container.UR.y;
            let ref mut fresh39 = *cvs.offset(cn as isize);
            *fresh39 = *((*e).vs).offset((n + 2 as libc::c_int * i) as isize);
            let ref mut fresh40 = *cvs.offset((cn + 1 as libc::c_int) as isize);
            *fresh40 = *((*e).vs).offset((n + 2 as libc::c_int * i + 1 as libc::c_int) as isize);
            (*cbb.offset(cn as isize)).LL.x = container.LL.x;
            (*cbb.offset(cn as isize)).LL.y = container.LL.y;
            (*cbb.offset(cn as isize)).UR.x = container.UR.x;
            (*cbb.offset(cn as isize)).UR.y = container.UR.y;
            (*cbb.offset((cn + 1 as libc::c_int) as isize)).LL.x = container.LL.x;
            (*cbb.offset((cn + 1 as libc::c_int) as isize)).LL.y = container.LL.y;
            (*cbb.offset((cn + 1 as libc::c_int) as isize)).UR.x = container.UR.x;
            (*cbb.offset((cn + 1 as libc::c_int) as isize)).UR.y = container.UR.y;
            if k == 0 as libc::c_int {
                (*cbb.offset(cn as isize)).UR.x = container.LL.x + 0.0001f64;
                (*cbb.offset((cn + 1 as libc::c_int) as isize)).LL.x = container.UR.x - 0.0001f64;
                *cm.offset(i as isize) = genXConstraints(
                    cn + 2 as libc::c_int,
                    cbb,
                    cvs,
                    &mut *cscl.offset(i as isize),
                    transitiveClosure,
                );
            } else {
                (*cbb.offset(cn as isize)).UR.y = container.LL.y + 0.0001f64;
                (*cbb.offset((cn + 1 as libc::c_int) as isize)).LL.y = container.UR.y - 0.0001f64;
                *cm.offset(i as isize) = genYConstraints(
                    cn + 2 as libc::c_int,
                    cbb,
                    cvs,
                    &mut *cscl.offset(i as isize),
                );
            }
            mol += *cm.offset(i as isize);
            free(cvs as *mut libc::c_void);
            free(cbb as *mut libc::c_void);
            i += 1;
        }
        let mut cn_0: libc::c_int = (*(*opt).clusters).ntoplevel + (*(*opt).clusters).nclusters;
        let mut cvs_0: *mut *mut Variable = gcalloc(
            cn_0 as size_t,
            ::std::mem::size_of::<*mut Variable>() as libc::c_ulong,
        ) as *mut *mut Variable;
        let mut cbb_0: *mut boxf = gcalloc(
            cn_0 as size_t,
            ::std::mem::size_of::<boxf>() as libc::c_ulong,
        ) as *mut boxf;
        i = 0 as libc::c_int;
        while i < (*(*opt).clusters).ntoplevel {
            let mut iv_0: libc::c_int = *((*(*opt).clusters).toplevel).offset(i as isize);
            let ref mut fresh41 = *cvs_0.offset(i as isize);
            *fresh41 = *((*e).vs).offset(iv_0 as isize);
            (*cbb_0.offset(i as isize)).LL.x = (*bb.offset(iv_0 as isize)).LL.x;
            (*cbb_0.offset(i as isize)).LL.y = (*bb.offset(iv_0 as isize)).LL.y;
            (*cbb_0.offset(i as isize)).UR.x = (*bb.offset(iv_0 as isize)).UR.x;
            (*cbb_0.offset(i as isize)).UR.y = (*bb.offset(iv_0 as isize)).UR.y;
            i += 1;
        }
        i = (*(*opt).clusters).ntoplevel;
        while i < cn_0 {
            let ref mut fresh42 = *cvs_0.offset(i as isize);
            *fresh42 = newVariable(
                123 as libc::c_int + i,
                1 as libc::c_int as libc::c_double,
                1 as libc::c_int as libc::c_double,
            );
            j = i - (*(*opt).clusters).ntoplevel;
            (*cbb_0.offset(i as isize)).LL.x = (*((*(*opt).clusters).bb).offset(j as isize)).LL.x;
            (*cbb_0.offset(i as isize)).LL.y = (*((*(*opt).clusters).bb).offset(j as isize)).LL.y;
            (*cbb_0.offset(i as isize)).UR.x = (*((*(*opt).clusters).bb).offset(j as isize)).UR.x;
            (*cbb_0.offset(i as isize)).UR.y = (*((*(*opt).clusters).bb).offset(j as isize)).UR.y;
            i += 1;
        }
        i = (*(*opt).clusters).nclusters;
        if k == 0 as libc::c_int {
            *cm.offset(i as isize) = genXConstraints(
                cn_0,
                cbb_0,
                cvs_0,
                &mut *cscl.offset(i as isize),
                transitiveClosure,
            );
        } else {
            *cm.offset(i as isize) =
                genYConstraints(cn_0, cbb_0, cvs_0, &mut *cscl.offset(i as isize));
        }
        i = (*(*opt).clusters).ntoplevel;
        while i < cn_0 {
            let mut dgap: libc::c_double = 0.;
            j = i - (*(*opt).clusters).ntoplevel;
            if k == 0 as libc::c_int {
                dgap =
                    -((*cbb_0.offset(i as isize)).UR.x - (*cbb_0.offset(i as isize)).LL.x) / 2.0f64;
            } else {
                dgap =
                    -((*cbb_0.offset(i as isize)).UR.y - (*cbb_0.offset(i as isize)).LL.y) / 2.0f64;
            }
            remapInConstraints(
                *cvs_0.offset(i as isize),
                *((*e).vs).offset((n + 2 as libc::c_int * j) as isize),
                dgap,
            );
            remapOutConstraints(
                *cvs_0.offset(i as isize),
                *((*e).vs).offset((n + 2 as libc::c_int * j + 1 as libc::c_int) as isize),
                dgap,
            );
            deleteVariable(*cvs_0.offset(i as isize));
            i += 1;
        }
        mol += *cm.offset((*(*opt).clusters).nclusters as isize);
        free(cvs_0 as *mut libc::c_void);
        free(cbb_0 as *mut libc::c_void);
        csol = newConstraints(mol);
        csolptr = csol;
        i = 0 as libc::c_int;
        while i < (*(*opt).clusters).nclusters + 1 as libc::c_int {
            j = 0 as libc::c_int;
            while j < *cm.offset(i as isize) {
                let fresh43 = csolptr;
                csolptr = csolptr.offset(1);
                *fresh43 = *(*cscl.offset(i as isize)).offset(j as isize);
                j += 1;
            }
            deleteConstraints(0 as libc::c_int, *cscl.offset(i as isize));
            i += 1;
        }
        free(cscl as *mut libc::c_void);
        free(cm as *mut libc::c_void);
    } else if k == 0 as libc::c_int {
        mol = genXConstraints(n, bb, (*e).vs, &mut csol, transitiveClosure);
    } else {
        mol = genYConstraints(n, bb, (*e).vs, &mut csol);
    }
    if (*e).m > 0 as libc::c_int {
        deleteVPSC((*e).vpsc);
        i = (*e).gm;
        while i < (*e).m {
            deleteConstraint(*((*e).cs).offset(i as isize));
            i += 1;
        }
        if (*e).cs != (*e).gcs {
            deleteConstraints(0 as libc::c_int, (*e).cs);
        }
    }
    if (*e).gm == 0 as libc::c_int {
        (*e).m = mol;
        let ref mut fresh44 = (*e).cs;
        *fresh44 = csol;
    } else {
        (*e).m = mol + (*e).gm;
        let ref mut fresh45 = (*e).cs;
        *fresh45 = newConstraints((*e).m);
        i = 0 as libc::c_int;
        while i < (*e).m {
            if i < (*e).gm {
                let ref mut fresh46 = *((*e).cs).offset(i as isize);
                *fresh46 = *((*e).gcs).offset(i as isize);
            } else {
                let ref mut fresh47 = *((*e).cs).offset(i as isize);
                *fresh47 = *csol.offset((i - (*e).gm) as isize);
            }
            i += 1;
        }
        deleteConstraints(0 as libc::c_int, csol);
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"  generated %d constraints\n\0" as *const u8 as *const libc::c_char,
            (*e).m,
        );
    }
    let ref mut fresh48 = (*e).vpsc;
    *fresh48 = newIncVPSC((*e).nv + (*e).nldv + (*e).ndv, (*e).vs, (*e).m, (*e).cs);
    free(bb as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn removeoverlaps(
    mut n: libc::c_int,
    mut coords: *mut *mut libc::c_float,
    mut opt: *mut ipsep_options,
) {
    let mut i: libc::c_int = 0;
    let mut e: *mut CMajEnvVPSC = initCMajVPSC(
        n,
        0 as *mut libc::c_float,
        0 as *mut vtx_data,
        opt,
        0 as libc::c_int,
    );
    generateNonoverlapConstraints(
        e,
        1.0f64 as libc::c_float,
        coords,
        0 as libc::c_int,
        1 as libc::c_int != 0,
        opt,
    );
    solveVPSC((*e).vpsc);
    i = 0 as libc::c_int;
    while i < n {
        *(*coords.offset(0 as libc::c_int as isize)).offset(i as isize) =
            getVariablePos(*((*e).vs).offset(i as isize)) as libc::c_float;
        i += 1;
    }
    generateNonoverlapConstraints(
        e,
        1.0f64 as libc::c_float,
        coords,
        1 as libc::c_int,
        0 as libc::c_int != 0,
        opt,
    );
    solveVPSC((*e).vpsc);
    i = 0 as libc::c_int;
    while i < n {
        *(*coords.offset(1 as libc::c_int as isize)).offset(i as isize) =
            getVariablePos(*((*e).vs).offset(i as isize)) as libc::c_float;
        i += 1;
    }
    deleteCMajEnvVPSC(e);
}
#[no_mangle]
pub unsafe extern "C" fn assign_digcola_levels(
    mut ordering: *mut libc::c_int,
    mut n: libc::c_int,
    mut level_inds: *mut libc::c_int,
    mut num_divisions: libc::c_int,
) -> *mut DigColaLevel {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: *mut DigColaLevel = gcalloc(
        (num_divisions + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<DigColaLevel>() as libc::c_ulong,
    ) as *mut DigColaLevel;
    (*l.offset(0 as libc::c_int as isize)).num_nodes =
        *level_inds.offset(0 as libc::c_int as isize);
    let ref mut fresh49 = (*l.offset(0 as libc::c_int as isize)).nodes;
    *fresh49 = gcalloc(
        (*l.offset(0 as libc::c_int as isize)).num_nodes as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*l.offset(0 as libc::c_int as isize)).num_nodes {
        *((*l.offset(0 as libc::c_int as isize)).nodes).offset(i as isize) =
            *ordering.offset(i as isize);
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < num_divisions {
        (*l.offset(i as isize)).num_nodes =
            *level_inds.offset(i as isize) - *level_inds.offset((i - 1 as libc::c_int) as isize);
        let ref mut fresh50 = (*l.offset(i as isize)).nodes;
        *fresh50 = gcalloc(
            (*l.offset(i as isize)).num_nodes as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        j = 0 as libc::c_int;
        while j < (*l.offset(i as isize)).num_nodes {
            *((*l.offset(i as isize)).nodes).offset(j as isize) = *ordering
                .offset((*level_inds.offset((i - 1 as libc::c_int) as isize) + j) as isize);
            j += 1;
        }
        i += 1;
    }
    if num_divisions > 0 as libc::c_int {
        (*l.offset(num_divisions as isize)).num_nodes =
            n - *level_inds.offset((num_divisions - 1 as libc::c_int) as isize);
        let ref mut fresh51 = (*l.offset(num_divisions as isize)).nodes;
        *fresh51 = gcalloc(
            (*l.offset(num_divisions as isize)).num_nodes as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        i = 0 as libc::c_int;
        while i < (*l.offset(num_divisions as isize)).num_nodes {
            *((*l.offset(num_divisions as isize)).nodes).offset(i as isize) = *ordering.offset(
                (*level_inds.offset((num_divisions - 1 as libc::c_int) as isize) + i) as isize,
            );
            i += 1;
        }
    }
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn delete_digcola_levels(
    mut l: *mut DigColaLevel,
    mut num_levels: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_levels {
        free((*l.offset(i as isize)).nodes as *mut libc::c_void);
        i += 1;
    }
    free(l as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn print_digcola_levels(
    mut logfile: *mut FILE,
    mut levels: *mut DigColaLevel,
    mut num_levels: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    fprintf(logfile, b"levels:\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < num_levels {
        fprintf(
            logfile,
            b"  l[%d]:\0" as *const u8 as *const libc::c_char,
            i,
        );
        j = 0 as libc::c_int;
        while j < (*levels.offset(i as isize)).num_nodes {
            fprintf(
                logfile,
                b"%d \0" as *const u8 as *const libc::c_char,
                *((*levels.offset(i as isize)).nodes).offset(j as isize),
            );
            j += 1;
        }
        fprintf(logfile, b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_num_digcola_constraints(
    mut levels: *mut DigColaLevel,
    mut num_levels: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut nc: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < num_levels {
        nc += (*levels.offset(i as isize)).num_nodes
            + (*levels.offset((i - 1 as libc::c_int) as isize)).num_nodes;
        i += 1;
    }
    nc += (*levels.offset(0 as libc::c_int as isize)).num_nodes
        + (*levels.offset((num_levels - 1 as libc::c_int) as isize)).num_nodes;
    return nc;
}
