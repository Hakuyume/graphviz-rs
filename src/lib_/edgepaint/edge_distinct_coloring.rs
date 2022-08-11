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
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn free(_: *mut libc::c_void);
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut Verbose: libc::c_uchar;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn SparseMatrix_new(
        m: libc::c_int,
        n: libc::c_int,
        nz: libc::c_int,
        type_0: libc::c_int,
        format: libc::c_int,
    ) -> SparseMatrix;
    fn SparseMatrix_from_coordinate_format(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_delete(A: SparseMatrix);
    fn SparseMatrix_coordinate_form_add_entry(
        A: SparseMatrix,
        irn: libc::c_int,
        jcn: libc::c_int,
        val: *mut libc::c_void,
    ) -> SparseMatrix;
    fn node_distinct_coloring(
        color_scheme: *mut libc::c_char,
        lightness: *mut libc::c_char,
        weightedQ: bool,
        A: SparseMatrix,
        accuracy: libc::c_double,
        seed: libc::c_int,
        cdim: *mut libc::c_int,
        colors: *mut *mut libc::c_double,
    ) -> libc::c_int;
    fn attach_edge_colors(g: *mut Agraph_t, dim: libc::c_int, colors: *mut libc::c_double);
    fn SparseMatrix_import_dot(
        g: *mut Agraph_t,
        dim: libc::c_int,
        label_sizes: *mut *mut libc::c_double,
        x: *mut *mut libc::c_double,
        n_edge_label_nodes: *mut libc::c_int,
        edge_label_nodes: *mut *mut libc::c_int,
        format: libc::c_int,
    ) -> SparseMatrix;
    fn Import_dot_splines(
        g: *mut Agraph_t,
        ne: *mut libc::c_int,
        xsplines: *mut *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn intersection_angle(
        p1: *mut libc::c_double,
        p2: *mut libc::c_double,
        q1: *mut libc::c_double,
        q2: *mut libc::c_double,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uint64_t = __uint64_t;
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
pub type Dtlink_t = _dtlink_s;
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
pub type Dtdisc_t = _dtdisc_s;
pub type Dt_t = _dt_s;
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
pub type Dict_t = _dt_s;
pub type IDTYPE = uint64_t;
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
pub type Agtag_t = Agtag_s;
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
pub type Agobj_t = Agobj_s;
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
pub type Agraph_t = Agraph_s;
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const FORMAT_COORD: C2RustUnnamed_2 = 2;
pub const FORMAT_CSR: C2RustUnnamed_2 = 1;
pub const FORMAT_CSC: C2RustUnnamed_2 = 0;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const MATRIX_TYPE_UNKNOWN: C2RustUnnamed_3 = 16;
pub const MATRIX_TYPE_PATTERN: C2RustUnnamed_3 = 8;
pub const MATRIX_TYPE_INTEGER: C2RustUnnamed_3 = 4;
pub const MATRIX_TYPE_COMPLEX: C2RustUnnamed_3 = 2;
pub const MATRIX_TYPE_REAL: C2RustUnnamed_3 = 1;
unsafe extern "C" fn splines_intersect(
    mut dim: libc::c_int,
    mut u1: libc::c_int,
    mut v1: libc::c_int,
    mut u2: libc::c_int,
    mut v2: libc::c_int,
    mut cos_critical: libc::c_double,
    mut check_edges_with_same_endpoint: libc::c_int,
    mut xsplines1: *mut libc::c_char,
    mut xsplines2: *mut libc::c_char,
) -> libc::c_int {
    let mut itmp: libc::c_int = 0;
    let mut len1: libc::c_int = 100 as libc::c_int;
    let mut len2: libc::c_int = 100 as libc::c_int;
    let mut x1: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut x2: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ns1: libc::c_int = 0 as libc::c_int;
    let mut ns2: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut iter1: libc::c_int = 0 as libc::c_int;
    let mut iter2: libc::c_int = 0 as libc::c_int;
    let mut cos_a: libc::c_double = 0.;
    let mut tmp: [libc::c_double; 2] = [0.; 2];
    let mut endp1: libc::c_int = 0 as libc::c_int;
    let mut endp2: libc::c_int = 0 as libc::c_int;
    tmp[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    tmp[0 as libc::c_int as usize] = tmp[1 as libc::c_int as usize];
    x1 = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(len1 as libc::c_ulong),
    ) as *mut libc::c_double;
    x2 = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(len2 as libc::c_ulong),
    ) as *mut libc::c_double;
    if dim <= 3 as libc::c_int {
    } else {
        __assert_fail(
            b"dim <= 3\0" as *const u8 as *const libc::c_char,
            b"edge_distinct_coloring.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 76], &[libc::c_char; 76]>(
                b"int splines_intersect(int, int, int, int, int, double, int, char *, char *)\0",
            ))
            .as_ptr(),
        );
    }
    if u1 == v2 {
        itmp = u2;
        u2 = v2;
        v2 = itmp;
    } else if v1 == u2 {
        itmp = u1;
        u1 = v1;
        v1 = itmp;
    } else if v1 == v2 {
        itmp = u2;
        u2 = v2;
        v2 = itmp;
        itmp = u1;
        u1 = v1;
        v1 = itmp;
    }
    if !xsplines1.is_null() {
        if !(strstr(xsplines1, b"e,\0" as *const u8 as *const libc::c_char)).is_null() {
            endp1 = 1 as libc::c_int;
            xsplines1 = (strstr(xsplines1, b"e,\0" as *const u8 as *const libc::c_char))
                .offset(2 as libc::c_int as isize);
        } else if !(strstr(xsplines2, b"s,\0" as *const u8 as *const libc::c_char)).is_null() {
            xsplines1 = (strstr(xsplines1, b"s,\0" as *const u8 as *const libc::c_char))
                .offset(2 as libc::c_int as isize);
        }
    }
    while !xsplines1.is_null()
        && sscanf(
            xsplines1,
            b"%lf,%lf\0" as *const u8 as *const libc::c_char,
            &mut *x1.offset((ns1 * dim) as isize) as *mut libc::c_double,
            &mut *x1.offset((ns1 * dim + 1 as libc::c_int) as isize) as *mut libc::c_double,
        ) == 2 as libc::c_int
    {
        if endp1 != 0 && iter1 == 0 as libc::c_int {
            tmp[0 as libc::c_int as usize] = *x1.offset((ns1 * dim) as isize);
            tmp[1 as libc::c_int as usize] = *x1.offset((ns1 * dim + 1 as libc::c_int) as isize);
        } else {
            ns1 += 1;
        }
        iter1 += 1;
        xsplines1 = strchr(xsplines1, ' ' as i32);
        if xsplines1.is_null() {
            break;
        }
        xsplines1 = xsplines1.offset(1);
        if ns1 * dim >= len1 {
            len1 = ns1 * dim
                + (if 10 as libc::c_int as libc::c_double
                    > 0.2f64 * ns1 as libc::c_double * dim as libc::c_double
                {
                    10 as libc::c_int as libc::c_double
                } else {
                    0.2f64 * ns1 as libc::c_double * dim as libc::c_double
                }) as libc::c_int;
            x1 = grealloc(
                x1 as *mut libc::c_void,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(len1 as libc::c_ulong),
            ) as *mut libc::c_double;
        }
    }
    if endp1 != 0 {
        ns1 += 1;
        if ns1 * dim >= len1 {
            len1 = ns1 * dim
                + (if 10 as libc::c_int as libc::c_double
                    > 0.2f64 * ns1 as libc::c_double * dim as libc::c_double
                {
                    10 as libc::c_int as libc::c_double
                } else {
                    0.2f64 * ns1 as libc::c_double * dim as libc::c_double
                }) as libc::c_int;
            x1 = grealloc(
                x1 as *mut libc::c_void,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(len1 as libc::c_ulong),
            ) as *mut libc::c_double;
        }
        *x1.offset(((ns1 - 1 as libc::c_int) * dim) as isize) = tmp[0 as libc::c_int as usize];
        *x1.offset(((ns1 - 1 as libc::c_int) * dim + 1 as libc::c_int) as isize) =
            tmp[1 as libc::c_int as usize];
    }
    if !xsplines2.is_null() {
        if !(strstr(xsplines2, b"e,\0" as *const u8 as *const libc::c_char)).is_null() {
            endp2 = 1 as libc::c_int;
            xsplines2 = (strstr(xsplines2, b"e,\0" as *const u8 as *const libc::c_char))
                .offset(2 as libc::c_int as isize);
        } else if !(strstr(xsplines2, b"s,\0" as *const u8 as *const libc::c_char)).is_null() {
            xsplines2 = (strstr(xsplines2, b"s,\0" as *const u8 as *const libc::c_char))
                .offset(2 as libc::c_int as isize);
        }
    }
    while !xsplines2.is_null()
        && sscanf(
            xsplines2,
            b"%lf,%lf\0" as *const u8 as *const libc::c_char,
            &mut *x2.offset((ns2 * dim) as isize) as *mut libc::c_double,
            &mut *x2.offset((ns2 * dim + 1 as libc::c_int) as isize) as *mut libc::c_double,
        ) == 2 as libc::c_int
    {
        if endp2 != 0 && iter2 == 0 as libc::c_int {
            tmp[0 as libc::c_int as usize] = *x2.offset((ns2 * dim) as isize);
            tmp[1 as libc::c_int as usize] = *x2.offset((ns2 * dim + 1 as libc::c_int) as isize);
        } else {
            ns2 += 1;
        }
        iter2 += 1;
        xsplines2 = strchr(xsplines2, ' ' as i32);
        if xsplines2.is_null() {
            break;
        }
        xsplines2 = xsplines2.offset(1);
        if ns2 * dim >= len2 {
            len2 = ns2 * dim
                + (if 10 as libc::c_int as libc::c_double
                    > 0.2f64 * ns2 as libc::c_double * dim as libc::c_double
                {
                    10 as libc::c_int as libc::c_double
                } else {
                    0.2f64 * ns2 as libc::c_double * dim as libc::c_double
                }) as libc::c_int;
            x2 = grealloc(
                x2 as *mut libc::c_void,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(len2 as libc::c_ulong),
            ) as *mut libc::c_double;
        }
    }
    if endp2 != 0 {
        ns2 += 1;
        if ns2 * dim >= len2 {
            len2 = ns2 * dim
                + (if 10 as libc::c_int as libc::c_double
                    > 0.2f64 * ns2 as libc::c_double * dim as libc::c_double
                {
                    10 as libc::c_int as libc::c_double
                } else {
                    0.2f64 * ns2 as libc::c_double * dim as libc::c_double
                }) as libc::c_int;
            x2 = grealloc(
                x2 as *mut libc::c_void,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(len2 as libc::c_ulong),
            ) as *mut libc::c_double;
        }
        *x2.offset(((ns2 - 1 as libc::c_int) * dim) as isize) = tmp[0 as libc::c_int as usize];
        *x2.offset(((ns2 - 1 as libc::c_int) * dim + 1 as libc::c_int) as isize) =
            tmp[1 as libc::c_int as usize];
    }
    i = 0 as libc::c_int;
    while i < ns1 - 1 as libc::c_int {
        j = 0 as libc::c_int;
        while j < ns2 - 1 as libc::c_int {
            cos_a = intersection_angle(
                &mut *x1.offset((dim * i) as isize),
                &mut *x1.offset((dim * (i + 1 as libc::c_int)) as isize),
                &mut *x2.offset((dim * j) as isize),
                &mut *x2.offset((dim * (j + 1 as libc::c_int)) as isize),
            );
            if check_edges_with_same_endpoint == 0 && cos_a >= -(1 as libc::c_int) as libc::c_double
            {
                cos_a = fabs(cos_a);
            }
            if cos_a > cos_critical {
                return 1 as libc::c_int;
            }
            j += 1;
        }
        i += 1;
    }
    free(x1 as *mut libc::c_void);
    free(x2 as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn edge_distinct_coloring(
    mut color_scheme: *mut libc::c_char,
    mut lightness: *mut libc::c_char,
    mut g: *mut Agraph_t,
    mut angle: libc::c_double,
    mut accuracy: libc::c_double,
    mut check_edges_with_same_endpoint: libc::c_int,
    mut seed: libc::c_int,
) -> *mut Agraph_t {
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dim: libc::c_int = 2 as libc::c_int;
    let mut A: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut C: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut irn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jcn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut nz2: libc::c_int = 0 as libc::c_int;
    let mut cos_critical: libc::c_double =
        cos(angle / 180 as libc::c_int as libc::c_double * 3.14159f64);
    let mut cos_a: libc::c_double = 0.;
    let mut u1: libc::c_int = 0;
    let mut v1: libc::c_int = 0;
    let mut u2: libc::c_int = 0;
    let mut v2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut colors: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut flag: libc::c_int = 0;
    let mut ne: libc::c_int = 0;
    let mut xsplines: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut cdim: libc::c_int = 0;
    A = SparseMatrix_import_dot(
        g,
        dim,
        0 as *mut *mut libc::c_double,
        &mut x,
        0 as *mut libc::c_int,
        0 as *mut *mut libc::c_int,
        FORMAT_COORD as libc::c_int,
    );
    if x.is_null() {
        fprintf(
            stderr,
            b"The gv file contains no or improper 2D coordinates\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut Agraph_t;
    }
    irn = (*A).ia;
    jcn = (*A).ja;
    nz = (*A).nz;
    i = 0 as libc::c_int;
    while i < nz {
        if *irn.offset(i as isize) != *jcn.offset(i as isize) {
            *irn.offset(nz2 as isize) = *irn.offset(i as isize);
            let fresh0 = nz2;
            nz2 = nz2 + 1;
            *jcn.offset(fresh0 as isize) = *jcn.offset(i as isize);
        }
        i += 1;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"cos = %f, nz2 = %d\n\0" as *const u8 as *const libc::c_char,
            cos_critical,
            nz2,
        );
    }
    B = SparseMatrix_new(
        nz2,
        nz2,
        1 as libc::c_int,
        MATRIX_TYPE_REAL as libc::c_int,
        FORMAT_COORD as libc::c_int,
    );
    if Import_dot_splines(g, &mut ne, &mut xsplines) != 0 {
        if ne == nz2 {
        } else {
            __assert_fail(
                b"ne == nz2\0" as *const u8 as *const libc::c_char,
                b"edge_distinct_coloring.c\0" as *const u8 as *const libc::c_char,
                196 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 87],
                    &[libc::c_char; 87],
                >(
                    b"Agraph_t *edge_distinct_coloring(char *, char *, Agraph_t *, double, double, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        cos_a = 1.0f64;
        i = 0 as libc::c_int;
        while i < nz2 {
            u1 = *irn.offset(i as isize);
            v1 = *jcn.offset(i as isize);
            j = i + 1 as libc::c_int;
            while j < nz2 {
                u2 = *irn.offset(j as isize);
                v2 = *jcn.offset(j as isize);
                if splines_intersect(
                    dim,
                    u1,
                    v1,
                    u2,
                    v2,
                    cos_critical,
                    check_edges_with_same_endpoint,
                    *xsplines.offset(i as isize),
                    *xsplines.offset(j as isize),
                ) != 0
                {
                    B = SparseMatrix_coordinate_form_add_entry(
                        B,
                        i,
                        j,
                        &mut cos_a as *mut libc::c_double as *mut libc::c_void,
                    );
                }
                j += 1;
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < nz2 {
            u1 = *irn.offset(i as isize);
            v1 = *jcn.offset(i as isize);
            j = i + 1 as libc::c_int;
            while j < nz2 {
                u2 = *irn.offset(j as isize);
                v2 = *jcn.offset(j as isize);
                cos_a = intersection_angle(
                    &mut *x.offset((dim * u1) as isize),
                    &mut *x.offset((dim * v1) as isize),
                    &mut *x.offset((dim * u2) as isize),
                    &mut *x.offset((dim * v2) as isize),
                );
                if check_edges_with_same_endpoint == 0
                    && cos_a >= -(1 as libc::c_int) as libc::c_double
                {
                    cos_a = fabs(cos_a);
                }
                if cos_a > cos_critical {
                    B = SparseMatrix_coordinate_form_add_entry(
                        B,
                        i,
                        j,
                        &mut cos_a as *mut libc::c_double as *mut libc::c_void,
                    );
                }
                j += 1;
            }
            i += 1;
        }
    }
    C = SparseMatrix_from_coordinate_format(B);
    if B != C {
        SparseMatrix_delete(B);
    }
    let mut weightedQ: libc::c_int = 0 as libc::c_int;
    flag = node_distinct_coloring(
        color_scheme,
        lightness,
        weightedQ != 0,
        C,
        accuracy,
        seed,
        &mut cdim,
        &mut colors,
    );
    if !(flag != 0) {
        if Verbose != 0 {
            fprintf(
                stderr,
                b"The edge conflict graph has %d nodes and %d edges\n\0" as *const u8
                    as *const libc::c_char,
                (*C).m,
                (*C).nz,
            );
        }
        attach_edge_colors(g, cdim, colors);
    }
    SparseMatrix_delete(A);
    SparseMatrix_delete(C);
    free(colors as *mut libc::c_void);
    free(x as *mut libc::c_void);
    if !xsplines.is_null() {
        i = 0 as libc::c_int;
        while i < ne {
            free(*xsplines.offset(i as isize) as *mut libc::c_void);
            i += 1;
        }
        free(xsplines as *mut libc::c_void);
    }
    return g;
}
