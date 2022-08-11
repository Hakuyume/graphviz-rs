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
    fn fabs(_: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn IMDS_given_dim(
        _: *mut vtx_data,
        _: libc::c_int,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
        _: libc::c_double,
    ) -> libc::c_int;
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
    fn compute_y_coords(
        _: *mut vtx_data,
        _: libc::c_int,
        _: *mut libc::c_double,
        _: libc::c_int,
    ) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    static mut Epsilon: libc::c_double;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn start_timer();
    fn elapsed_sec() -> libc::c_double;
    fn stress_majorization_kD_mkernel(
        graph: *mut vtx_data,
        n: libc::c_int,
        coords: *mut *mut libc::c_double,
        nodes: *mut *mut node_t,
        dim: libc::c_int,
        opts: libc::c_int,
        model: libc::c_int,
        maxi: libc::c_int,
    ) -> libc::c_int;
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
    fn distance_kD(
        _: *mut *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn conjugate_gradient_mkernel(
        _: *mut libc::c_float,
        _: *mut libc::c_float,
        _: *mut libc::c_float,
        _: libc::c_int,
        _: libc::c_double,
        _: libc::c_int,
    ) -> libc::c_int;
    fn initConstrainedMajorization(
        _: *mut libc::c_float,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
    ) -> *mut CMajEnv;
    fn constrained_majorization_new_with_gaps(
        _: *mut CMajEnv,
        _: *mut libc::c_float,
        _: *mut *mut libc::c_float,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_float,
        _: libc::c_float,
    ) -> libc::c_int;
    fn deleteCMajEnv(e: *mut CMajEnv);
    fn unpackMatrix(packedMat: *mut libc::c_float, n: libc::c_int) -> *mut *mut libc::c_float;
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
pub type Dtsearch_f =
    Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, libc::c_int) -> *mut libc::c_void>;
pub type Dtmemory_f = Option<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, size_t, *mut Dtdisc_t) -> *mut libc::c_void,
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
pub type Dtevent_f = Option<
    unsafe extern "C" fn(*mut Dt_t, libc::c_int, *mut libc::c_void, *mut Dtdisc_t) -> libc::c_int,
>;
pub type Dthash_f =
    Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> libc::c_uint>;
pub type Dtcompar_f = Option<
    unsafe extern "C" fn(
        *mut Dt_t,
        *mut libc::c_void,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> libc::c_int,
>;
pub type Dtfree_f = Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> ()>;
pub type Dtmake_f =
    Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> *mut libc::c_void>;
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
pub type agobjfn_t =
    Option<unsafe extern "C" fn(*mut Agraph_t, *mut Agobj_t, *mut libc::c_void) -> ()>;
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
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
pub type agobjupdfn_t = Option<
    unsafe extern "C" fn(*mut Agraph_t, *mut Agobj_t, *mut libc::c_void, *mut Agsym_t) -> (),
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
    pub afread: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub putstr: Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int>,
    pub flush: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
}
pub type Agiddisc_t = Agiddisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agiddisc_s {
    pub open: Option<unsafe extern "C" fn(*mut Agraph_t, *mut Agdisc_t) -> *mut libc::c_void>,
    pub map: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_char,
            *mut IDTYPE,
            libc::c_int,
        ) -> libc::c_long,
    >,
    pub alloc: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> libc::c_long>,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> ()>,
    pub print:
        Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> *mut libc::c_char>,
    pub close: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub idregister:
        Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, *mut libc::c_void) -> ()>,
}
pub type Agmemdisc_t = Agmemdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agmemdisc_s {
    pub open: Option<unsafe extern "C" fn(*mut Agdisc_t) -> *mut libc::c_void>,
    pub alloc: Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void>,
    pub resize: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            size_t,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    pub close: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type Agdesc_t = Agdesc_s;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
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
pub unsafe extern "C" fn stress_majorization_with_hierarchy(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut d_coords: *mut *mut libc::c_double,
    mut nodes: *mut *mut node_t,
    mut dim: libc::c_int,
    mut opts: libc::c_int,
    mut model: libc::c_int,
    mut maxi: libc::c_int,
    mut levels_gap: libc::c_double,
) -> libc::c_int {
    let mut current_block: u64;
    let mut iterations: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut directionalityExist: bool = 0 as libc::c_int != 0;
    let mut lap1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut dist_accumulator: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut tmp_coords: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut b: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut degrees: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut lap2: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut lap_length: libc::c_int = 0;
    let mut f_storage: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut coords: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut conj_tol: libc::c_double = 1e-3f64;
    let mut unpackedLap: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut cMajEnv: *mut CMajEnv = 0 as *mut CMajEnv;
    let mut y_0: libc::c_double = 0.;
    let mut length: libc::c_int = 0;
    let mut smart_ini: libc::c_int = opts & 0x4 as libc::c_int;
    let mut diameter: DistType = 0;
    let mut Dij: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut abs_tol: libc::c_double = 1e-2f64;
    let mut relative_tol: libc::c_double = 1e-1f64;
    let mut ordering: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut levels: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut constant_term: libc::c_float = 0.;
    let mut count: libc::c_int = 0;
    let mut degree: libc::c_double = 0.;
    let mut step: libc::c_int = 0;
    let mut val: libc::c_float = 0.;
    let mut old_stress: libc::c_double = 0.;
    let mut new_stress: libc::c_double = 0.;
    let mut converged: bool = false;
    let mut len: libc::c_int = 0;
    let mut num_levels: libc::c_int = 0;
    let mut hierarchy_boundaries: *mut libc::c_float = 0 as *mut libc::c_float;
    if !((*graph.offset(0 as libc::c_int as isize)).edists).is_null() {
        i = 0 as libc::c_int;
        while i < n {
            j = 1 as libc::c_int;
            while j < (*graph.offset(i as isize)).nedges {
                directionalityExist = (directionalityExist as libc::c_int
                    | (*((*graph.offset(i as isize)).edists).offset(j as isize)
                        != 0 as libc::c_int as libc::c_float) as libc::c_int)
                    as bool;
                j += 1;
            }
            i += 1;
        }
    }
    if !directionalityExist {
        return stress_majorization_kD_mkernel(graph, n, d_coords, nodes, dim, opts, model, maxi);
    }
    if smart_ini != 0 {
        let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut y: *mut libc::c_double = 0 as *mut libc::c_double;
        if dim > 2 as libc::c_int {
            if stress_majorization_kD_mkernel(
                graph,
                n,
                d_coords.offset(1 as libc::c_int as isize),
                nodes,
                dim - 1 as libc::c_int,
                opts,
                model,
                15 as libc::c_int,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            i = 0 as libc::c_int;
            while i < n {
                *(*d_coords.offset((dim - 1 as libc::c_int) as isize)).offset(i as isize) =
                    *(*d_coords.offset(1 as libc::c_int as isize)).offset(i as isize);
                i += 1;
            }
        }
        x = *d_coords.offset(0 as libc::c_int as isize);
        y = *d_coords.offset(1 as libc::c_int as isize);
        if compute_y_coords(graph, n, y, n) != 0 {
            iterations = -(1 as libc::c_int);
            current_block = 13361066454504989531;
        } else if compute_hierarchy(
            graph,
            n,
            abs_tol,
            relative_tol,
            y,
            &mut ordering,
            &mut levels,
            &mut num_levels,
        ) != 0
        {
            iterations = -(1 as libc::c_int);
            current_block = 13361066454504989531;
        } else {
            if num_levels < 1 as libc::c_int {
                return stress_majorization_kD_mkernel(
                    graph, n, d_coords, nodes, dim, opts, model, maxi,
                );
            }
            if levels_gap > 0 as libc::c_int as libc::c_double {
                let mut displacement: libc::c_double = 0 as libc::c_int as libc::c_double;
                let mut stop: libc::c_int = 0;
                i = 0 as libc::c_int;
                while i < num_levels {
                    displacement += if 0 as libc::c_int as libc::c_double
                        > levels_gap
                            - (*y.offset(
                                *ordering.offset(*levels.offset(i as isize) as isize) as isize
                            ) + displacement
                                - *y.offset(*ordering.offset(
                                    (*levels.offset(i as isize) - 1 as libc::c_int) as isize,
                                ) as isize))
                    {
                        0 as libc::c_int as libc::c_double
                    } else {
                        levels_gap
                            - (*y.offset(
                                *ordering.offset(*levels.offset(i as isize) as isize) as isize
                            ) + displacement
                                - *y.offset(*ordering.offset(
                                    (*levels.offset(i as isize) - 1 as libc::c_int) as isize,
                                ) as isize))
                    };
                    stop = if i < num_levels - 1 as libc::c_int {
                        *levels.offset((i + 1 as libc::c_int) as isize)
                    } else {
                        n
                    };
                    j = *levels.offset(i as isize);
                    while j < stop {
                        *y.offset(*ordering.offset(j as isize) as isize) += displacement;
                        j += 1;
                    }
                    i += 1;
                }
            }
            if dim == 2 as libc::c_int {
                if IMDS_given_dim(graph, n, y, x, Epsilon) != 0 {
                    iterations = -(1 as libc::c_int);
                    current_block = 13361066454504989531;
                } else {
                    current_block = 15462640364611497761;
                }
            } else {
                current_block = 15462640364611497761;
            }
        }
    } else {
        initLayout(graph, n, dim, d_coords, nodes);
        if compute_hierarchy(
            graph,
            n,
            abs_tol,
            relative_tol,
            0 as *mut libc::c_double,
            &mut ordering,
            &mut levels,
            &mut num_levels,
        ) != 0
        {
            iterations = -(1 as libc::c_int);
            current_block = 13361066454504989531;
        } else {
            current_block = 15462640364611497761;
        }
    }
    match current_block {
        15462640364611497761 => {
            if n == 1 as libc::c_int {
                return 0 as libc::c_int;
            }
            hierarchy_boundaries = gcalloc(
                num_levels as size_t,
                ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
            ) as *mut libc::c_float;
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
            if smart_ini == 0 {
                let mut max: libc::c_double = 1 as libc::c_int as libc::c_double;
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
                        *(*d_coords.offset(i as isize)).offset(j as isize) *=
                            10 as libc::c_int as libc::c_double / max;
                        j += 1;
                    }
                    i += 1;
                }
            }
            if levels_gap > 0 as libc::c_int as libc::c_double {
                let mut length_0: libc::c_int = n + n * (n - 1 as libc::c_int) / 2 as libc::c_int;
                let mut sum1: libc::c_double = 0.;
                let mut sum2: libc::c_double = 0.;
                let mut scale_ratio: libc::c_double = 0.;
                let mut count_0: libc::c_int = 0;
                sum1 = (n * (n - 1 as libc::c_int) / 2 as libc::c_int) as libc::c_float
                    as libc::c_double;
                sum2 = 0 as libc::c_int as libc::c_double;
                count_0 = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < n - 1 as libc::c_int {
                    count_0 += 1;
                    j = i + 1 as libc::c_int;
                    while j < n {
                        sum2 += distance_kD(d_coords, dim, i, j)
                            / *Dij.offset(count_0 as isize) as libc::c_double;
                        j += 1;
                        count_0 += 1;
                    }
                    i += 1;
                }
                scale_ratio = sum2 / sum1;
                i = 0 as libc::c_int;
                while i < length_0 {
                    *Dij.offset(i as isize) *= scale_ratio as libc::c_float;
                    i += 1;
                }
            }
            i = 0 as libc::c_int;
            while i < dim {
                orthog1(n, *d_coords.offset(i as isize));
                i += 1;
            }
            y_0 = *(*d_coords.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize);
            i = 0 as libc::c_int;
            while i < n {
                *(*d_coords.offset(1 as libc::c_int as isize)).offset(i as isize) -= y_0;
                i += 1;
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
                let ref mut fresh0 = *coords.offset(i as isize);
                *fresh0 = f_storage.offset((i * n) as isize);
                j = 0 as libc::c_int;
                while j < n {
                    *(*coords.offset(i as isize)).offset(j as isize) =
                        *(*d_coords.offset(i as isize)).offset(j as isize) as libc::c_float;
                    j += 1;
                }
                i += 1;
            }
            constant_term = (n * (n - 1 as libc::c_int) / 2 as libc::c_int) as libc::c_float;
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
            b = gcalloc(
                dim as size_t,
                ::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong,
            ) as *mut *mut libc::c_float;
            let ref mut fresh1 = *b.offset(0 as libc::c_int as isize);
            *fresh1 = gcalloc(
                (dim * n) as size_t,
                ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
            ) as *mut libc::c_float;
            k = 1 as libc::c_int;
            while k < dim {
                let ref mut fresh2 = *b.offset(k as isize);
                *fresh2 = (*b.offset(0 as libc::c_int as isize)).offset((k * n) as isize);
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
            lap1 = gcalloc(
                lap_length as size_t,
                ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
            ) as *mut libc::c_float;
            old_stress = 1.7976931348623157e+308f64;
            unpackedLap = unpackMatrix(lap2, n);
            cMajEnv = initConstrainedMajorization(lap2, n, ordering, levels, num_levels);
            converged = 0 as libc::c_int != 0;
            iterations = 0 as libc::c_int;
            's_997: loop {
                if !(iterations < maxi && !converged) {
                    current_block = 5790857258831596080;
                    break;
                }
                set_vector_val(n, 0 as libc::c_int as libc::c_double, degrees);
                sqrt_vecf(lap_length, lap2, lap1);
                count = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < n - 1 as libc::c_int {
                    len = n - i - 1 as libc::c_int;
                    set_vector_valf(n, 0 as libc::c_int as libc::c_float, dist_accumulator);
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
                        vectors_additionf(len, tmp_coords, dist_accumulator, dist_accumulator);
                        k += 1;
                    }
                    invert_sqrt_vec(len, dist_accumulator);
                    j = 0 as libc::c_int;
                    while j < len {
                        if *dist_accumulator.offset(j as isize) >= 3.40282347e+38f32
                            || *dist_accumulator.offset(j as isize)
                                < 0 as libc::c_int as libc::c_float
                        {
                            *dist_accumulator.offset(j as isize) =
                                0 as libc::c_int as libc::c_float;
                        }
                        j += 1;
                    }
                    count += 1;
                    degree = 0 as libc::c_int as libc::c_double;
                    j = 0 as libc::c_int;
                    while j < len {
                        let ref mut fresh3 = *lap1.offset(count as isize);
                        *fresh3 *= *dist_accumulator.offset(j as isize);
                        val = *fresh3;
                        degree += val as libc::c_double;
                        *degrees.offset((i + j + 1 as libc::c_int) as isize) -=
                            val as libc::c_double;
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
                    *lap1.offset(count as isize) = *degrees.offset(i as isize) as libc::c_float;
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
                    new_stress += vectors_inner_productf(
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
                    right_mult_with_vector_ff(lap2, n, *coords.offset(k as isize), tmp_coords);
                    new_stress -= vectors_inner_productf(n, *coords.offset(k as isize), tmp_coords);
                    k += 1;
                }
                converged = fabs(new_stress - old_stress) / fabs(old_stress + 1e-10f64) < Epsilon;
                converged = (converged as libc::c_int
                    | (iterations > 1 as libc::c_int && new_stress > old_stress) as libc::c_int)
                    as bool;
                old_stress = new_stress;
                k = 0 as libc::c_int;
                while k < dim {
                    if k == 1 as libc::c_int {
                        constrained_majorization_new_with_gaps(
                            cMajEnv,
                            *b.offset(k as isize),
                            coords,
                            dim,
                            k,
                            15 as libc::c_int,
                            hierarchy_boundaries,
                            levels_gap as libc::c_float,
                        );
                    } else if conjugate_gradient_mkernel(
                        lap2,
                        *coords.offset(k as isize),
                        *b.offset(k as isize),
                        n,
                        conj_tol,
                        n,
                    ) != 0
                    {
                        iterations = -(1 as libc::c_int);
                        current_block = 13361066454504989531;
                        break 's_997;
                    }
                    k += 1;
                }
                iterations += 1;
            }
            match current_block {
                13361066454504989531 => {}
                _ => {
                    free(hierarchy_boundaries as *mut libc::c_void);
                    deleteCMajEnv(cMajEnv);
                    if !coords.is_null() {
                        i = 0 as libc::c_int;
                        while i < dim {
                            j = 0 as libc::c_int;
                            while j < n {
                                *(*d_coords.offset(i as isize)).offset(j as isize) =
                                    *(*coords.offset(i as isize)).offset(j as isize)
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
                }
            }
        }
        _ => {}
    }
    free(ordering as *mut libc::c_void);
    free(levels as *mut libc::c_void);
    if !unpackedLap.is_null() {
        free(*unpackedLap.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        free(unpackedLap as *mut libc::c_void);
    }
    return iterations;
}
