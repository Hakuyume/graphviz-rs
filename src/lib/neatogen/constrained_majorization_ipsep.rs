#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type VPSC;
    pub type Constraint;
    pub type Variable;
    fn fabs(_: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    static mut Epsilon: libc::c_double;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn start_timer();
    fn elapsed_sec() -> libc::c_double;
    fn compute_apsp_packed(graph: *mut vtx_data, n: libc::c_int) -> *mut libc::c_float;
    fn compute_apsp_artifical_weights_packed(
        graph: *mut vtx_data,
        n: libc::c_int,
    ) -> *mut libc::c_float;
    fn circuitModel(graph: *mut vtx_data, nG: libc::c_int) -> *mut libc::c_float;
    fn mdsModel(graph: *mut vtx_data, nG: libc::c_int) -> *mut libc::c_float;
    fn initLayout(
        graph: *mut vtx_data,
        n: libc::c_int,
        dim: libc::c_int,
        coords: *mut *mut libc::c_double,
        nodes: *mut *mut node_t,
    ) -> libc::c_int;
    fn orthog1(n: libc::c_int, vec: *mut libc::c_double);
    fn right_mult_with_vector_ff(
        _: *mut libc::c_float,
        _: libc::c_int,
        _: *mut libc::c_float,
        _: *mut libc::c_float,
    );
    fn vectors_additionf(
        n: libc::c_int,
        vector1: *mut libc::c_float,
        vector2: *mut libc::c_float,
        result: *mut libc::c_float,
    );
    fn vectors_mult_additionf(
        n: libc::c_int,
        vector1: *mut libc::c_float,
        alpha: libc::c_float,
        vector2: *mut libc::c_float,
    );
    fn vectors_inner_productf(
        n: libc::c_int,
        vector1: *mut libc::c_float,
        vector2: *mut libc::c_float,
    ) -> libc::c_double;
    fn set_vector_val(n: libc::c_int, val: libc::c_double, result: *mut libc::c_double);
    fn set_vector_valf(n: libc::c_int, val: libc::c_float, result: *mut libc::c_float);
    fn square_vec(n: libc::c_int, vec: *mut libc::c_float);
    fn invert_vec(n: libc::c_int, vec: *mut libc::c_float);
    fn sqrt_vecf(n: libc::c_int, source: *mut libc::c_float, target: *mut libc::c_float);
    fn invert_sqrt_vec(n: libc::c_int, vec: *mut libc::c_float);
    fn conjugate_gradient_mkernel(
        _: *mut libc::c_float,
        _: *mut libc::c_float,
        _: *mut libc::c_float,
        _: libc::c_int,
        _: libc::c_double,
        _: libc::c_int,
    ) -> libc::c_int;
    fn initCMajVPSC(
        n: libc::c_int,
        packedMat: *mut libc::c_float,
        graph: *mut vtx_data,
        opt: *mut ipsep_options,
        diredges: libc::c_int,
    ) -> *mut CMajEnvVPSC;
    fn constrained_majorization_vpsc(
        _: *mut CMajEnvVPSC,
        _: *mut libc::c_float,
        _: *mut libc::c_float,
        _: libc::c_int,
    ) -> libc::c_int;
    fn deleteCMajEnvVPSC(e: *mut CMajEnvVPSC);
    fn generateNonoverlapConstraints(
        e: *mut CMajEnvVPSC,
        nsizeScale: libc::c_float,
        coords: *mut *mut libc::c_float,
        k: libc::c_int,
        transitiveClosure: bool,
        opt: *mut ipsep_options,
    );
    fn removeoverlaps(_: libc::c_int, _: *mut *mut libc::c_float, _: *mut ipsep_options);
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
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
pub type Agnode_t = Agnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agnode_s {
    pub base: Agobj_t,
    pub root: *mut Agraph_t,
    pub mainsub: Agsubnode_t,
}
pub type Agsubnode_t = Agsubnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agsubnode_s {
    pub seq_link: Dtlink_t,
    pub id_link: Dtlink_t,
    pub node: *mut Agnode_t,
    pub in_id: *mut Dtlink_t,
    pub out_id: *mut Dtlink_t,
    pub in_seq: *mut Dtlink_t,
    pub out_seq: *mut Dtlink_t,
}
pub type Dtlink_t = _dtlink_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtlink_s {
    pub right: *mut Dtlink_t,
    pub hl: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _hash: libc::c_uint,
    pub _left: *mut Dtlink_t,
}
pub type Agraph_t = Agraph_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agraph_s {
    pub base: Agobj_t,
    pub desc: Agdesc_t,
    pub link: Dtlink_t,
    pub n_seq: *mut Dict_t,
    pub n_id: *mut Dict_t,
    pub e_seq: *mut Dict_t,
    pub e_id: *mut Dict_t,
    pub g_dict: *mut Dict_t,
    pub parent: *mut Agraph_t,
    pub root: *mut Agraph_t,
    pub clos: *mut Agclos_t,
}
pub type Agclos_t = Agclos_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agclos_s {
    pub disc: Agdisc_t,
    pub state: Agdstate_t,
    pub strdict: *mut Dict_t,
    pub seq: [uint64_t; 3],
    pub cb: *mut Agcbstack_t,
    pub callbacks_enabled: libc::c_uchar,
    pub lookup_by_name: [*mut Dict_t; 3],
    pub lookup_by_id: [*mut Dict_t; 3],
}
pub type Dict_t = _dt_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dt_s {
    pub searchf: Dtsearch_f,
    pub disc: *mut Dtdisc_t,
    pub data: *mut Dtdata_t,
    pub memoryf: Dtmemory_f,
    pub meth: *mut Dtmethod_t,
    pub type_0: libc::c_int,
    pub nview: libc::c_int,
    pub view: *mut Dt_t,
    pub walk: *mut Dt_t,
    pub user: *mut libc::c_void,
}
pub type Dt_t = _dt_s;
pub type Dtmethod_t = _dtmethod_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtmethod_s {
    pub searchf: Dtsearch_f,
    pub type_0: libc::c_int,
}
pub type Dtsearch_f = Option::<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, libc::c_int) -> *mut libc::c_void,
>;
pub type Dtmemory_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        *mut libc::c_void,
        size_t,
        *mut Dtdisc_t,
    ) -> *mut libc::c_void,
>;
pub type Dtdisc_t = _dtdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtdisc_s {
    pub key: libc::c_int,
    pub size: libc::c_int,
    pub link: libc::c_int,
    pub makef: Dtmake_f,
    pub freef: Dtfree_f,
    pub comparf: Dtcompar_f,
    pub hashf: Dthash_f,
    pub memoryf: Dtmemory_f,
    pub eventf: Dtevent_f,
}
pub type Dtevent_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        libc::c_int,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> libc::c_int,
>;
pub type Dthash_f = Option::<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> libc::c_uint,
>;
pub type Dtcompar_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        *mut libc::c_void,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> libc::c_int,
>;
pub type Dtfree_f = Option::<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> (),
>;
pub type Dtmake_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> *mut libc::c_void,
>;
pub type Dtdata_t = _dtdata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtdata_s {
    pub type_0: libc::c_int,
    pub here: *mut Dtlink_t,
    pub hh: C2RustUnnamed_0,
    pub ntab: libc::c_int,
    pub size: libc::c_int,
    pub loop_0: libc::c_int,
    pub minp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _htab: *mut *mut Dtlink_t,
    pub _head: *mut Dtlink_t,
}
pub type Agcbstack_t = Agcbstack_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agcbstack_s {
    pub f: *mut Agcbdisc_t,
    pub state: *mut libc::c_void,
    pub prev: *mut Agcbstack_t,
}
pub type Agcbdisc_t = Agcbdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agcbdisc_s {
    pub graph: C2RustUnnamed_1,
    pub node: C2RustUnnamed_1,
    pub edge: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ins: agobjfn_t,
    pub mod_0: agobjupdfn_t,
    pub del: agobjfn_t,
}
pub type agobjfn_t = Option::<
    unsafe extern "C" fn(*mut Agraph_t, *mut Agobj_t, *mut libc::c_void) -> (),
>;
pub type Agobj_t = Agobj_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agobj_s {
    pub tag: Agtag_t,
    pub data: *mut Agrec_t,
}
pub type Agrec_t = Agrec_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agrec_s {
    pub name: *mut libc::c_char,
    pub next: *mut Agrec_t,
}
pub type Agtag_t = Agtag_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Agtag_s {
    #[bitfield(name = "objtype", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "mtflock", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "attrwf", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "seq", ty = "libc::c_uint", bits = "4..=31")]
    pub objtype_mtflock_attrwf_seq: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub id: IDTYPE,
}
pub type IDTYPE = uint64_t;
pub type agobjupdfn_t = Option::<
    unsafe extern "C" fn(
        *mut Agraph_t,
        *mut Agobj_t,
        *mut libc::c_void,
        *mut Agsym_t,
    ) -> (),
>;
pub type Agsym_t = Agsym_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agsym_s {
    pub link: Dtlink_t,
    pub name: *mut libc::c_char,
    pub defval: *mut libc::c_char,
    pub id: libc::c_int,
    pub kind: libc::c_uchar,
    pub fixed: libc::c_uchar,
    pub print: libc::c_uchar,
}
pub type Agdstate_t = Agdstate_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agdstate_s {
    pub mem: *mut libc::c_void,
    pub id: *mut libc::c_void,
}
pub type Agdisc_t = Agdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agdisc_s {
    pub mem: *mut Agmemdisc_t,
    pub id: *mut Agiddisc_t,
    pub io: *mut Agiodisc_t,
}
pub type Agiodisc_t = Agiodisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agiodisc_s {
    pub afread: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub putstr: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
    >,
    pub flush: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
}
pub type Agiddisc_t = Agiddisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agiddisc_s {
    pub open: Option::<
        unsafe extern "C" fn(*mut Agraph_t, *mut Agdisc_t) -> *mut libc::c_void,
    >,
    pub map: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_char,
            *mut IDTYPE,
            libc::c_int,
        ) -> libc::c_long,
    >,
    pub alloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> libc::c_long,
    >,
    pub free: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> (),
    >,
    pub print: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> *mut libc::c_char,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub idregister: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, *mut libc::c_void) -> (),
    >,
}
pub type Agmemdisc_t = Agmemdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agmemdisc_s {
    pub open: Option::<unsafe extern "C" fn(*mut Agdisc_t) -> *mut libc::c_void>,
    pub alloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub resize: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            size_t,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    pub close: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type Agdesc_t = Agdesc_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Agdesc_s {
    #[bitfield(name = "directed", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "strict", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "no_loop", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "maingraph", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "flatlock", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "no_write", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "has_attrs", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "has_cmpnd", ty = "libc::c_uint", bits = "7..=7")]
    pub directed_strict_no_loop_maingraph_flatlock_no_write_has_attrs_has_cmpnd: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type node_t = Agnode_s;
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
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
#[no_mangle]
pub unsafe extern "C" fn stress_majorization_cola(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut nedges_graph: libc::c_int,
    mut d_coords: *mut *mut libc::c_double,
    mut nodes: *mut *mut node_t,
    mut dim: libc::c_int,
    mut model: libc::c_int,
    mut maxi: libc::c_int,
    mut opt: *mut ipsep_options,
) -> libc::c_int {
    let mut current_block: u64;
    let mut iterations: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lap1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut dist_accumulator: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut tmp_coords: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut b: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut degrees: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut lap2: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut lap_length: libc::c_int = 0;
    let mut f_storage: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut coords: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut orig_n: libc::c_int = n;
    let mut cMajEnvHor: *mut CMajEnvVPSC = 0 as *mut CMajEnvVPSC;
    let mut cMajEnvVrt: *mut CMajEnvVPSC = 0 as *mut CMajEnvVPSC;
    let mut y_0: libc::c_double = 0.;
    let mut length: libc::c_int = 0;
    let mut diameter: DistType = 0;
    let mut Dij: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut constant_term: libc::c_float = 0.;
    let mut count: libc::c_int = 0;
    let mut degree: libc::c_double = 0.;
    let mut step: libc::c_int = 0;
    let mut val: libc::c_float = 0.;
    let mut old_stress: libc::c_double = 0.;
    let mut new_stress: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut converged: bool = false;
    let mut len: libc::c_int = 0;
    let mut nsizeScale: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut maxEdgeLen: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut max: libc::c_double = 1 as libc::c_int as libc::c_double;
    initLayout(graph, n, dim, d_coords, nodes);
    if n == 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < n {
        j = 1 as libc::c_int;
        while j < (*graph.offset(i as isize)).nedges {
            maxEdgeLen = if *((*graph.offset(i as isize)).ewgts).offset(j as isize)
                > maxEdgeLen
            {
                *((*graph.offset(i as isize)).ewgts).offset(j as isize)
            } else {
                maxEdgeLen
            };
            j += 1;
        }
        i += 1;
    }
    if maxi == 0 as libc::c_int {
        return iterations;
    }
    if Verbose != 0 {
        start_timer();
    }
    if model == 2 as libc::c_int {
        if Verbose != 0 {
            fprintf(
                stderr,
                b"Calculating subset model\0" as *const u8 as *const libc::c_char,
            );
        }
        Dij = compute_apsp_artifical_weights_packed(graph, n);
    } else if model == 1 as libc::c_int {
        Dij = circuitModel(graph, n);
        if Dij.is_null() {
            agerr(
                AGWARN,
                b"graph is disconnected. Hence, the circuit model\n\0" as *const u8
                    as *const libc::c_char,
            );
            agerr(
                AGPREV,
                b"is undefined. Reverting to the shortest path model.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else if model == 3 as libc::c_int {
        if Verbose != 0 {
            fprintf(
                stderr,
                b"Calculating MDS model\0" as *const u8 as *const libc::c_char,
            );
        }
        Dij = mdsModel(graph, n);
    }
    if Dij.is_null() {
        if Verbose != 0 {
            fprintf(
                stderr,
                b"Calculating shortest paths\0" as *const u8 as *const libc::c_char,
            );
        }
        Dij = compute_apsp_packed(graph, n);
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b": %.2f sec\n\0" as *const u8 as *const libc::c_char,
            elapsed_sec(),
        );
        fprintf(
            stderr,
            b"Setting initial positions\0" as *const u8 as *const libc::c_char,
        );
        start_timer();
    }
    diameter = -(1 as libc::c_int);
    length = n + n * (n - 1 as libc::c_int) / 2 as libc::c_int;
    i = 0 as libc::c_int;
    while i < length {
        if *Dij.offset(i as isize) > diameter as libc::c_float {
            diameter = *Dij.offset(i as isize) as libc::c_int;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < dim {
        j = 0 as libc::c_int;
        while j < n {
            if fabs(*(*d_coords.offset(i as isize)).offset(j as isize)) > max {
                max = fabs(*(*d_coords.offset(i as isize)).offset(j as isize));
            }
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < dim {
        j = 0 as libc::c_int;
        while j < n {
            *(*d_coords.offset(i as isize)).offset(j as isize)
                *= 10 as libc::c_int as libc::c_double / max;
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < dim {
        orthog1(n, *d_coords.offset(i as isize));
        i += 1;
    }
    y_0 = *(*d_coords.offset(1 as libc::c_int as isize))
        .offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < n {
        *(*d_coords.offset(1 as libc::c_int as isize)).offset(i as isize) -= y_0;
        i += 1;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b": %.2f sec\0" as *const u8 as *const libc::c_char,
            elapsed_sec(),
        );
    }
    lap2 = Dij;
    lap_length = n + n * (n - 1 as libc::c_int) / 2 as libc::c_int;
    square_vec(lap_length, lap2);
    invert_vec(lap_length, lap2);
    if (*(*opt).clusters).nclusters > 0 as libc::c_int {
        let mut nn: libc::c_int = n + (*(*opt).clusters).nclusters * 2 as libc::c_int;
        let mut clap_length: libc::c_int = nn
            + nn * (nn - 1 as libc::c_int) / 2 as libc::c_int;
        let mut clap: *mut libc::c_float = gcalloc(
            clap_length as size_t,
            ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
        ) as *mut libc::c_float;
        let mut c0: libc::c_int = 0;
        let mut c1: libc::c_int = 0;
        let mut v: libc::c_float = 0.;
        c1 = 0 as libc::c_int;
        c0 = c1;
        i = 0 as libc::c_int;
        while i < nn {
            j = 0 as libc::c_int;
            while j < nn - i {
                if i < n && j < n - i {
                    let fresh0 = c0;
                    c0 = c0 + 1;
                    v = *lap2.offset(fresh0 as isize);
                } else if j == 1 as libc::c_int
                        && i % 2 as libc::c_int == 1 as libc::c_int
                    {
                    v = maxEdgeLen;
                    v *= v;
                    if v > 0.01f32 {
                        v = 1.0f32 / v;
                    }
                } else {
                    v = 0 as libc::c_int as libc::c_float;
                }
                let fresh1 = c1;
                c1 = c1 + 1;
                *clap.offset(fresh1 as isize) = v;
                j += 1;
            }
            i += 1;
        }
        free(lap2 as *mut libc::c_void);
        lap2 = clap;
        n = nn;
        lap_length = clap_length;
    }
    count = 0 as libc::c_int;
    degrees = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    set_vector_val(n, 0 as libc::c_int as libc::c_double, degrees);
    i = 0 as libc::c_int;
    while i < n - 1 as libc::c_int {
        degree = 0 as libc::c_int as libc::c_double;
        count += 1;
        j = 1 as libc::c_int;
        while j < n - i {
            val = *lap2.offset(count as isize);
            degree += val as libc::c_double;
            *degrees.offset((i + j) as isize) -= val as libc::c_double;
            j += 1;
            count += 1;
        }
        *degrees.offset(i as isize) -= degree;
        i += 1;
    }
    step = n;
    count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *lap2.offset(count as isize) = *degrees.offset(i as isize) as libc::c_float;
        i += 1;
        count += step;
        step -= 1;
    }
    coords = gcalloc(
        dim as size_t,
        ::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong,
    ) as *mut *mut libc::c_float;
    f_storage = gcalloc(
        (dim * n) as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    i = 0 as libc::c_int;
    while i < dim {
        let ref mut fresh2 = *coords.offset(i as isize);
        *fresh2 = f_storage.offset((i * n) as isize);
        j = 0 as libc::c_int;
        while j < n {
            *(*coords.offset(i as isize))
                .offset(
                    j as isize,
                ) = if j < orig_n {
                *(*d_coords.offset(i as isize)).offset(j as isize) as libc::c_float
            } else {
                0 as libc::c_int as libc::c_float
            };
            j += 1;
        }
        i += 1;
    }
    constant_term = (n * (n - 1 as libc::c_int) / 2 as libc::c_int) as libc::c_float;
    b = gcalloc(
        dim as size_t,
        ::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong,
    ) as *mut *mut libc::c_float;
    let ref mut fresh3 = *b.offset(0 as libc::c_int as isize);
    *fresh3 = gcalloc(
        (dim * n) as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    k = 1 as libc::c_int;
    while k < dim {
        let ref mut fresh4 = *b.offset(k as isize);
        *fresh4 = (*b.offset(0 as libc::c_int as isize)).offset((k * n) as isize);
        k += 1;
    }
    tmp_coords = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    dist_accumulator = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    old_stress = 1.7976931348623157e+308f64;
    cMajEnvHor = initCMajVPSC(n, lap2, graph, opt, 0 as libc::c_int);
    if cMajEnvHor.is_null() {
        iterations = -(1 as libc::c_int);
    } else {
        cMajEnvVrt = initCMajVPSC(n, lap2, graph, opt, (*opt).diredges);
        if cMajEnvVrt.is_null() {
            iterations = -(1 as libc::c_int);
        } else {
            lap1 = gcalloc(
                lap_length as size_t,
                ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
            ) as *mut libc::c_float;
            converged = 0 as libc::c_int != 0;
            iterations = 0 as libc::c_int;
            loop {
                if !(iterations < maxi && !converged) {
                    current_block = 5565703735569783978;
                    break;
                }
                set_vector_val(n, 0 as libc::c_int as libc::c_double, degrees);
                sqrt_vecf(lap_length, lap2, lap1);
                count = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < n - 1 as libc::c_int {
                    len = n - i - 1 as libc::c_int;
                    set_vector_valf(
                        n,
                        0 as libc::c_int as libc::c_float,
                        dist_accumulator,
                    );
                    k = 0 as libc::c_int;
                    while k < dim {
                        set_vector_valf(
                            len,
                            *(*coords.offset(k as isize)).offset(i as isize),
                            tmp_coords,
                        );
                        vectors_mult_additionf(
                            len,
                            tmp_coords,
                            -(1 as libc::c_int) as libc::c_float,
                            (*coords.offset(k as isize))
                                .offset(i as isize)
                                .offset(1 as libc::c_int as isize),
                        );
                        square_vec(len, tmp_coords);
                        vectors_additionf(
                            len,
                            tmp_coords,
                            dist_accumulator,
                            dist_accumulator,
                        );
                        k += 1;
                    }
                    invert_sqrt_vec(len, dist_accumulator);
                    j = 0 as libc::c_int;
                    while j < len {
                        if *dist_accumulator.offset(j as isize) >= 3.40282347e+38f32
                            || *dist_accumulator.offset(j as isize)
                                < 0 as libc::c_int as libc::c_float
                        {
                            *dist_accumulator
                                .offset(j as isize) = 0 as libc::c_int as libc::c_float;
                        }
                        j += 1;
                    }
                    count += 1;
                    degree = 0 as libc::c_int as libc::c_double;
                    j = 0 as libc::c_int;
                    while j < len {
                        let ref mut fresh5 = *lap1.offset(count as isize);
                        *fresh5 *= *dist_accumulator.offset(j as isize);
                        val = *fresh5;
                        degree += val as libc::c_double;
                        *degrees.offset((i + j + 1 as libc::c_int) as isize)
                            -= val as libc::c_double;
                        j += 1;
                        count += 1;
                    }
                    *degrees.offset(i as isize) -= degree;
                    i += 1;
                }
                step = n;
                count = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < n {
                    *lap1
                        .offset(
                            count as isize,
                        ) = *degrees.offset(i as isize) as libc::c_float;
                    i += 1;
                    count += step;
                    step -= 1;
                }
                k = 0 as libc::c_int;
                while k < dim {
                    right_mult_with_vector_ff(
                        lap1,
                        n,
                        *coords.offset(k as isize),
                        *b.offset(k as isize),
                    );
                    k += 1;
                }
                new_stress = 0 as libc::c_int as libc::c_double;
                k = 0 as libc::c_int;
                while k < dim {
                    new_stress
                        += vectors_inner_productf(
                            n,
                            *coords.offset(k as isize),
                            *b.offset(k as isize),
                        );
                    k += 1;
                }
                new_stress *= 2 as libc::c_int as libc::c_double;
                new_stress += constant_term as libc::c_double;
                k = 0 as libc::c_int;
                while k < dim {
                    right_mult_with_vector_ff(
                        lap2,
                        n,
                        *coords.offset(k as isize),
                        tmp_coords,
                    );
                    new_stress
                        -= vectors_inner_productf(
                            n,
                            *coords.offset(k as isize),
                            tmp_coords,
                        );
                    k += 1;
                }
                if Verbose as libc::c_int != 0
                    && iterations % 1 as libc::c_int == 0 as libc::c_int
                {
                    fprintf(
                        stderr,
                        b"%.3f \0" as *const u8 as *const libc::c_char,
                        new_stress,
                    );
                    if iterations % 10 as libc::c_int == 0 as libc::c_int {
                        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    }
                }
                converged = new_stress < old_stress
                    && fabs(new_stress - old_stress) / fabs(old_stress + 1e-10f64)
                        < Epsilon;
                old_stress = new_stress;
                if (iterations >= maxi - 1 as libc::c_int
                    || converged as libc::c_int != 0)
                    && (*opt).noverlap == 1 as libc::c_int && nsizeScale < 0.999f64
                {
                    nsizeScale += 0.1f64;
                    if Verbose != 0 {
                        fprintf(
                            stderr,
                            b"nsizescale=%f,iterations=%d\n\0" as *const u8
                                as *const libc::c_char,
                            nsizeScale,
                            iterations,
                        );
                    }
                    iterations = 0 as libc::c_int;
                    converged = 0 as libc::c_int != 0;
                }
                if (*opt).noverlap == 1 as libc::c_int && nsizeScale > 0.001f64 {
                    generateNonoverlapConstraints(
                        cMajEnvHor,
                        nsizeScale as libc::c_float,
                        coords,
                        0 as libc::c_int,
                        nsizeScale >= 0.5f64,
                        opt,
                    );
                }
                if (*cMajEnvHor).m > 0 as libc::c_int {
                    constrained_majorization_vpsc(
                        cMajEnvHor,
                        *b.offset(0 as libc::c_int as isize),
                        *coords.offset(0 as libc::c_int as isize),
                        1000 as libc::c_int,
                    );
                } else if conjugate_gradient_mkernel(
                        lap2,
                        *coords.offset(0 as libc::c_int as isize),
                        *b.offset(0 as libc::c_int as isize),
                        n,
                        1e-3f64,
                        n,
                    ) < 0 as libc::c_int
                    {
                    iterations = -(1 as libc::c_int);
                    current_block = 15343090773358496180;
                    break;
                }
                if (*opt).noverlap == 1 as libc::c_int && nsizeScale > 0.001f64 {
                    generateNonoverlapConstraints(
                        cMajEnvVrt,
                        nsizeScale as libc::c_float,
                        coords,
                        1 as libc::c_int,
                        0 as libc::c_int != 0,
                        opt,
                    );
                }
                if (*cMajEnvVrt).m > 0 as libc::c_int {
                    if constrained_majorization_vpsc(
                        cMajEnvVrt,
                        *b.offset(1 as libc::c_int as isize),
                        *coords.offset(1 as libc::c_int as isize),
                        1000 as libc::c_int,
                    ) < 0 as libc::c_int
                    {
                        iterations = -(1 as libc::c_int);
                        current_block = 15343090773358496180;
                        break;
                    }
                } else {
                    conjugate_gradient_mkernel(
                        lap2,
                        *coords.offset(1 as libc::c_int as isize),
                        *b.offset(1 as libc::c_int as isize),
                        n,
                        1e-3f64,
                        n,
                    );
                }
                iterations += 1;
            }
            match current_block {
                15343090773358496180 => {}
                _ => {
                    if Verbose != 0 {
                        fprintf(
                            stderr,
                            b"\nfinal e = %f %d iterations %.2f sec\n\0" as *const u8
                                as *const libc::c_char,
                            new_stress,
                            iterations,
                            elapsed_sec(),
                        );
                    }
                    deleteCMajEnvVPSC(cMajEnvHor);
                    deleteCMajEnvVPSC(cMajEnvVrt);
                    if (*opt).noverlap == 2 as libc::c_int {
                        removeoverlaps(orig_n, coords, opt);
                    }
                }
            }
        }
    }
    if !coords.is_null() {
        i = 0 as libc::c_int;
        while i < dim {
            j = 0 as libc::c_int;
            while j < orig_n {
                *(*d_coords.offset(i as isize))
                    .offset(
                        j as isize,
                    ) = *(*coords.offset(i as isize)).offset(j as isize)
                    as libc::c_double;
                j += 1;
            }
            i += 1;
        }
        free(*coords.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        free(coords as *mut libc::c_void);
    }
    if !b.is_null() {
        free(*b.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        free(b as *mut libc::c_void);
    }
    free(tmp_coords as *mut libc::c_void);
    free(dist_accumulator as *mut libc::c_void);
    free(degrees as *mut libc::c_void);
    free(lap2 as *mut libc::c_void);
    free(lap1 as *mut libc::c_void);
    return iterations;
}
