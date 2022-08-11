#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn agsubg(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        cflag: libc::c_int,
    ) -> *mut Agraph_t;
    fn agnode(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agnode_t;
    fn agsubnode(
        g: *mut Agraph_t,
        n: *mut Agnode_t,
        createflag: libc::c_int,
    ) -> *mut Agnode_t;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agedge(
        g: *mut Agraph_t,
        t: *mut Agnode_t,
        h: *mut Agnode_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agsubedge(
        g: *mut Agraph_t,
        e: *mut Agedge_t,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstedge(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtedge(g: *mut Agraph_t, e: *mut Agedge_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agbindrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        recsize: libc::c_uint,
        move_to_front: libc::c_int,
    ) -> *mut libc::c_void;
    fn aginit(
        g: *mut Agraph_t,
        kind: libc::c_int,
        rec_name: *const libc::c_char,
        rec_size: libc::c_int,
        move_to_front: libc::c_int,
    );
    fn agread(chan: *mut libc::c_void, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agwrite(g: *mut Agraph_t, chan: *mut libc::c_void) -> libc::c_int;
    fn agisdirected(g: *mut Agraph_t) -> libc::c_int;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn agopen(
        name: *mut libc::c_char,
        desc: Agdesc_t,
        disc: *mut Agdisc_t,
    ) -> *mut Agraph_t;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agnedges(g: *mut Agraph_t) -> libc::c_int;
    fn agdegree(
        g: *mut Agraph_t,
        n: *mut Agnode_t,
        in_0: libc::c_int,
        out: libc::c_int,
    ) -> libc::c_int;
    static mut Agdirected: Agdesc_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn newIngraph(
        _: *mut ingraph_state,
        _: *mut *mut libc::c_char,
        _: opengfn,
    ) -> *mut ingraph_state;
    fn nextGraph(_: *mut ingraph_state) -> *mut Agraph_t;
    fn fileName(_: *mut ingraph_state) -> *mut libc::c_char;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    static mut optopt: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
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
pub type Dtevent_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        libc::c_int,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> libc::c_int,
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
pub type Dict_t = _dt_s;
pub type IDTYPE = uint64_t;
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
pub type agobjfn_t = Option::<
    unsafe extern "C" fn(*mut Agraph_t, *mut Agobj_t, *mut libc::c_void) -> (),
>;
pub type Agraph_t = Agraph_s;
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
pub type Agnode_t = Agnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agedge_s {
    pub base: Agobj_t,
    pub id_link: Dtlink_t,
    pub seq_link: Dtlink_t,
    pub node: *mut Agnode_t,
}
pub type Agedge_t = Agedge_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gv_stack_t {
    pub base: *mut *mut libc::c_void,
    pub size: size_t,
    pub capacity: size_t,
}
pub const FIRST_ALLOCATION: C2RustUnnamed_2 = 512;
pub type C2RustUnnamed_2 = libc::c_uint;
pub type opengfn = Option::<unsafe extern "C" fn(*mut FILE) -> *mut Agraph_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ingdisc {
    pub openf: Option::<unsafe extern "C" fn(*mut libc::c_char) -> *mut libc::c_void>,
    pub readf: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut Agraph_t>,
    pub closef: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub dflt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ingraph_state {
    pub u: C2RustUnnamed_3,
    pub ctr: libc::c_int,
    pub ingraphs: libc::c_int,
    pub fp: *mut libc::c_void,
    pub fns: *mut ingdisc,
    pub heap: bool,
    pub errors: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub Files: *mut *mut libc::c_char,
    pub Graphs: *mut *mut Agraph_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agraphinfo_t {
    pub h: Agrec_t,
    pub rep: *mut Agnode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agnodeinfo_t {
    pub h: Agrec_t,
    pub val: libc::c_uint,
    pub scc: *mut Agraph_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sccstate {
    pub Comp: libc::c_uint,
    pub ID: libc::c_uint,
    pub N_nodes_in_nontriv_SCC: libc::c_int,
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn stack_push_or_exit(
    mut stack: *mut gv_stack_t,
    mut item: *mut libc::c_void,
) {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void stack_push_or_exit(gv_stack_t *, void *)\0"))
                .as_ptr(),
        );
    }
    let mut r: libc::c_int = stack_push(stack, item);
    if (r != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"stack_push failed: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(r),
        );
        graphviz_exit(1 as libc::c_int);
    }
}
#[inline]
unsafe extern "C" fn stack_push(
    mut stack: *mut gv_stack_t,
    mut item: *mut libc::c_void,
) -> libc::c_int {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"int stack_push(gv_stack_t *, void *)\0"))
                .as_ptr(),
        );
    }
    if (*stack).size == (*stack).capacity {
        if ((18446744073709551615 as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong) < (*stack).capacity)
            as libc::c_int as libc::c_long != 0
        {
            return 75 as libc::c_int;
        }
        let mut c: size_t = if (*stack).capacity == 0 as libc::c_int as libc::c_ulong {
            FIRST_ALLOCATION as libc::c_int as libc::c_ulong
        } else {
            (2 as libc::c_int as libc::c_ulong).wrapping_mul((*stack).capacity)
        };
        let mut b: *mut *mut libc::c_void = realloc(
            (*stack).base as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong).wrapping_mul(c),
        ) as *mut *mut libc::c_void;
        if (b == 0 as *mut libc::c_void as *mut *mut libc::c_void) as libc::c_int
            as libc::c_long != 0
        {
            return 12 as libc::c_int;
        }
        (*stack).capacity = c;
        let ref mut fresh0 = (*stack).base;
        *fresh0 = b;
    }
    if !((*stack).base).is_null() {} else {
        __assert_fail(
            b"stack->base != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"int stack_push(gv_stack_t *, void *)\0"))
                .as_ptr(),
        );
    }
    if (*stack).capacity > (*stack).size {} else {
        __assert_fail(
            b"stack->capacity > stack->size\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"int stack_push(gv_stack_t *, void *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh1 = *((*stack).base).offset((*stack).size as isize);
    *fresh1 = item;
    let ref mut fresh2 = (*stack).size;
    *fresh2 = (*fresh2).wrapping_add(1);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn stack_is_empty(mut stack: *const gv_stack_t) -> bool {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"_Bool stack_is_empty(const gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    return stack_size(stack) == 0 as libc::c_int as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn stack_size(mut stack: *const gv_stack_t) -> size_t {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            23 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"size_t stack_size(const gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    return (*stack).size;
}
#[inline]
unsafe extern "C" fn stack_top(mut stack: *mut gv_stack_t) -> *mut libc::c_void {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void *stack_top(gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    if !stack_is_empty(stack)
        && !(b"access to top of an empty stack\0" as *const u8 as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"!stack_is_empty(stack) && \"access to top of an empty stack\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void *stack_top(gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    return *((*stack).base)
        .offset(
            ((*stack).size).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
}
#[inline]
unsafe extern "C" fn stack_pop(mut stack: *mut gv_stack_t) -> *mut libc::c_void {
    let mut top: *mut libc::c_void = stack_top(stack);
    let ref mut fresh3 = (*stack).size;
    *fresh3 = (*fresh3).wrapping_sub(1);
    return top;
}
#[inline]
unsafe extern "C" fn stack_reset(mut stack: *mut gv_stack_t) {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void stack_reset(gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    free((*stack).base as *mut libc::c_void);
    memset(
        stack as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<gv_stack_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn getrep(mut g: *mut Agraph_t) -> *mut Agnode_t {
    return (*((*g).base.data as *mut Agraphinfo_t)).rep;
}
unsafe extern "C" fn setrep(mut g: *mut Agraph_t, mut rep: *mut Agnode_t) {
    let ref mut fresh4 = (*((*g).base.data as *mut Agraphinfo_t)).rep;
    *fresh4 = rep;
}
unsafe extern "C" fn getscc(mut n: *mut Agnode_t) -> *mut Agraph_t {
    return (*((*n).base.data as *mut Agnodeinfo_t)).scc;
}
unsafe extern "C" fn setscc(mut n: *mut Agnode_t, mut scc: *mut Agraph_t) {
    let ref mut fresh5 = (*((*n).base.data as *mut Agnodeinfo_t)).scc;
    *fresh5 = scc;
}
unsafe extern "C" fn getval(mut n: *mut Agnode_t) -> libc::c_uint {
    return (*((*n).base.data as *mut Agnodeinfo_t)).val;
}
unsafe extern "C" fn setval(mut n: *mut Agnode_t, mut v: libc::c_uint) {
    (*((*n).base.data as *mut Agnodeinfo_t)).val = v;
}
static mut wantDegenerateComp: libc::c_int = 0;
static mut Silent: libc::c_int = 0;
static mut StatsOnly: libc::c_int = 0;
static mut Verbose: libc::c_int = 0;
static mut CmdName: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut Files: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut outfp: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn nodeInduce(mut g: *mut Agraph_t, mut map: *mut Agraph_t) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut rootg: *mut Agraph_t = agroot(g as *mut libc::c_void);
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(rootg, n);
        while !e.is_null() {
            if !(agsubnode(
                g,
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node,
                0 as libc::c_int,
            ))
                .is_null()
            {
                agsubedge(g, e, (0 as libc::c_int == 0) as libc::c_int);
            } else {
                let mut tscc: *mut Agraph_t = getscc(
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                        .node,
                );
                let mut hscc: *mut Agraph_t = getscc(
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                );
                if !tscc.is_null() && !hscc.is_null() {
                    agedge(
                        map,
                        getrep(tscc),
                        getrep(hscc),
                        0 as *mut libc::c_char,
                        (0 as libc::c_int == 0) as libc::c_int,
                    );
                }
            }
            e = agnxtout(rootg, e);
        }
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn visit(
    mut n: *mut Agnode_t,
    mut map: *mut Agraph_t,
    mut sp: *mut gv_stack_t,
    mut st: *mut sccstate,
) -> libc::c_uint {
    let mut m: libc::c_uint = 0;
    let mut min: libc::c_uint = 0;
    let mut t: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let ref mut fresh6 = (*st).ID;
    *fresh6 = (*fresh6).wrapping_add(1);
    min = *fresh6;
    setval(n, min);
    stack_push_or_exit(sp, n as *mut libc::c_void);
    e = agfstout((*n).root, n);
    while !e.is_null() {
        t = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node;
        if getval(t) == 0 as libc::c_int as libc::c_uint {
            m = visit(t, map, sp, st);
        } else {
            m = getval(t);
        }
        if m < min {
            min = m;
        }
        e = agnxtout((*n).root, e);
    }
    if getval(n) == min {
        if wantDegenerateComp == 0 && stack_top(sp) == n as *mut libc::c_void {
            setval(
                n,
                (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint),
            );
            stack_pop(sp);
        } else {
            let mut name: [libc::c_char; 32] = [0; 32];
            let mut G: *mut Agraph_t = agraphof(n as *mut libc::c_void);
            let ref mut fresh7 = (*st).Comp;
            let fresh8 = *fresh7;
            *fresh7 = (*fresh7).wrapping_add(1);
            snprintf(
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"cluster_%u\0" as *const u8 as *const libc::c_char,
                fresh8,
            );
            subg = agsubg(G, name.as_mut_ptr(), (0 as libc::c_int == 0) as libc::c_int);
            agbindrec(
                subg as *mut libc::c_void,
                b"scc_graph\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_uint,
                1 as libc::c_int,
            );
            setrep(
                subg,
                agnode(map, name.as_mut_ptr(), (0 as libc::c_int == 0) as libc::c_int),
            );
            loop {
                t = stack_pop(sp) as *mut Agnode_t;
                agsubnode(subg, t, (0 as libc::c_int == 0) as libc::c_int);
                setval(
                    t,
                    (2147483647 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_uint)
                        .wrapping_add(1 as libc::c_uint),
                );
                setscc(t, subg);
                let ref mut fresh9 = (*st).N_nodes_in_nontriv_SCC;
                *fresh9 += 1;
                if !(t != n) {
                    break;
                }
            }
            nodeInduce(subg, map);
            if StatsOnly == 0 {
                agwrite(subg, outfp as *mut libc::c_void);
            }
        }
    }
    return min;
}
unsafe extern "C" fn label(
    mut n: *mut Agnode_t,
    mut nodecnt: libc::c_int,
    mut edgecnt: *mut libc::c_int,
) -> libc::c_int {
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    setval(n, 1 as libc::c_int as libc::c_uint);
    nodecnt += 1;
    e = agfstedge((*n).root, n);
    while !e.is_null() {
        *edgecnt += 1 as libc::c_int;
        if (*e).node == n {
            e = if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e.offset(-(1 as libc::c_int as isize))
            } else {
                e.offset(1 as libc::c_int as isize)
            };
        }
        if getval((*e).node) == 0 {
            nodecnt = label((*e).node, nodecnt, edgecnt);
        }
        e = agnxtedge((*n).root, e, n);
    }
    return nodecnt;
}
unsafe extern "C" fn countComponents(
    mut g: *mut Agraph_t,
    mut max_degree: *mut libc::c_int,
    mut nontree_frac: *mut libc::c_float,
) -> libc::c_int {
    let mut nc: libc::c_int = 0 as libc::c_int;
    let mut sum_edges: libc::c_int = 0 as libc::c_int;
    let mut sum_nontree: libc::c_int = 0 as libc::c_int;
    let mut deg: libc::c_int = 0;
    let mut n_edges: libc::c_int = 0;
    let mut n_nodes: libc::c_int = 0;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    n = agfstnode(g);
    while !n.is_null() {
        if getval(n) == 0 {
            nc += 1;
            n_edges = 0 as libc::c_int;
            n_nodes = label(n, 0 as libc::c_int, &mut n_edges);
            sum_edges += n_edges;
            sum_nontree += n_edges - n_nodes + 1 as libc::c_int;
        }
        n = agnxtnode(g, n);
    }
    if !max_degree.is_null() {
        let mut maxd: libc::c_int = 0 as libc::c_int;
        n = agfstnode(g);
        while !n.is_null() {
            deg = agdegree(
                g,
                n,
                (0 as libc::c_int == 0) as libc::c_int,
                (0 as libc::c_int == 0) as libc::c_int,
            );
            if maxd < deg {
                maxd = deg;
            }
            setval(n, 0 as libc::c_int as libc::c_uint);
            n = agnxtnode(g, n);
        }
        *max_degree = maxd;
    }
    if !nontree_frac.is_null() {
        if sum_edges > 0 as libc::c_int {
            *nontree_frac = sum_nontree as libc::c_float / sum_edges as libc::c_float;
        } else {
            *nontree_frac = 0.0f64 as libc::c_float;
        }
    }
    return nc;
}
unsafe extern "C" fn process(mut G: *mut Agraph_t) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut map: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut nc: libc::c_int = 0 as libc::c_int;
    let mut nontree_frac: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut Maxdegree: libc::c_int = 0 as libc::c_int;
    let mut stack: gv_stack_t = {
        let mut init = gv_stack_t {
            base: 0 as *mut *mut libc::c_void,
            size: 0,
            capacity: 0,
        };
        init
    };
    let mut state: sccstate = sccstate {
        Comp: 0,
        ID: 0,
        N_nodes_in_nontriv_SCC: 0,
    };
    aginit(
        G,
        0 as libc::c_int,
        b"scc_graph\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    aginit(
        G,
        1 as libc::c_int,
        b"scc_node\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    state.ID = 0 as libc::c_int as libc::c_uint;
    state.Comp = state.ID;
    state.N_nodes_in_nontriv_SCC = 0 as libc::c_int;
    if Verbose != 0 {
        nc = countComponents(G, &mut Maxdegree, &mut nontree_frac);
    }
    map = agopen(
        b"scc_map\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Agdirected,
        0 as *mut Agdisc_t,
    );
    n = agfstnode(G);
    while !n.is_null() {
        if getval(n) == 0 as libc::c_int as libc::c_uint {
            visit(n, map, &mut stack, &mut state);
        }
        n = agnxtnode(G, n);
    }
    stack_reset(&mut stack);
    if StatsOnly == 0 {
        agwrite(map, outfp as *mut libc::c_void);
    }
    agclose(map);
    if Verbose != 0 {
        fprintf(
            stderr,
            b"%d %d %d %u %.4f %d %.4f\n\0" as *const u8 as *const libc::c_char,
            agnnodes(G),
            agnedges(G),
            nc,
            state.Comp,
            state.N_nodes_in_nontriv_SCC as libc::c_double
                / agnnodes(G) as libc::c_double,
            Maxdegree,
            nontree_frac as libc::c_double,
        );
    } else if Silent == 0 {
        fprintf(
            stderr,
            b"%d nodes, %d edges, %u strong components\n\0" as *const u8
                as *const libc::c_char,
            agnnodes(G),
            agnedges(G),
            state.Comp,
        );
    }
}
unsafe extern "C" fn openFile(mut name: *const libc::c_char) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fprintf(
            stderr,
            b"gvpack: could not open file %s for writing\n\0" as *const u8
                as *const libc::c_char,
            name,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return fp;
}
static mut useString: *mut libc::c_char = b"Usage: %s [-sdv?] <files>\n  -s           - only produce statistics\n  -S           - silent\n  -d           - allow degenerate components\n  -o<outfile>  - write to <outfile> (stdout)\n  -v           - verbose\n  -?           - print usage\nIf no files are specified, stdin is used\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn usage(mut v: libc::c_int) {
    printf(useString, CmdName);
    graphviz_exit(v);
}
unsafe extern "C" fn scanArgs(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    let mut c: libc::c_int = 0;
    CmdName = *argv.offset(0 as libc::c_int as isize);
    opterr = 0 as libc::c_int;
    loop {
        c = getopt(argc, argv, b":o:sdvS?\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            115 => {
                StatsOnly = 1 as libc::c_int;
            }
            100 => {
                wantDegenerateComp = 1 as libc::c_int;
            }
            111 => {
                if !outfp.is_null() {
                    fclose(outfp);
                }
                outfp = openFile(optarg);
            }
            118 => {
                Verbose = 1 as libc::c_int;
            }
            83 => {
                Verbose = 0 as libc::c_int;
                Silent = 1 as libc::c_int;
            }
            58 => {
                fprintf(
                    stderr,
                    b"%s: option -%c missing argument - ignored\n\0" as *const u8
                        as *const libc::c_char,
                    CmdName,
                    optopt,
                );
            }
            63 => {
                if optopt == '\0' as i32 || optopt == '?' as i32 {
                    usage(0 as libc::c_int);
                } else {
                    fprintf(
                        stderr,
                        b"%s: option -%c unrecognized\n\0" as *const u8
                            as *const libc::c_char,
                        CmdName,
                        optopt,
                    );
                    usage(1 as libc::c_int);
                }
            }
            _ => {}
        }
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if argc > 0 as libc::c_int {
        Files = argv;
    }
    if outfp.is_null() {
        outfp = stdout;
    }
}
unsafe extern "C" fn gread(mut fp: *mut FILE) -> *mut Agraph_t {
    return agread(fp as *mut libc::c_void, 0 as *mut Agdisc_t);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut ig: ingraph_state = ingraph_state {
        u: C2RustUnnamed_3 {
            Files: 0 as *mut *mut libc::c_char,
        },
        ctr: 0,
        ingraphs: 0,
        fp: 0 as *mut libc::c_void,
        fns: 0 as *mut ingdisc,
        heap: false,
        errors: 0,
    };
    scanArgs(argc, argv);
    newIngraph(
        &mut ig,
        Files,
        Some(gread as unsafe extern "C" fn(*mut FILE) -> *mut Agraph_t),
    );
    loop {
        g = nextGraph(&mut ig);
        if g.is_null() {
            break;
        }
        if agisdirected(g) != 0 {
            process(g);
        } else {
            fprintf(
                stderr,
                b"Graph %s in %s is undirected - ignored\n\0" as *const u8
                    as *const libc::c_char,
                agnameof(g as *mut libc::c_void),
                fileName(&mut ig),
            );
        }
        agclose(g);
    }
    graphviz_exit(0 as libc::c_int);
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
