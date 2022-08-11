#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn free(_: *mut libc::c_void);
    static mut Dtoset: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn dtwalk(
        _: *mut Dt_t,
        _: Option::<
            unsafe extern "C" fn(
                *mut Dt_t,
                *mut libc::c_void,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _grid {
    pub data: *mut Dt_t,
    pub cellMem: *mut block_t,
    pub cellCur: *mut block_t,
    pub listSize: libc::c_int,
    pub listMem: *mut node_list,
    pub listCur: *mut node_list,
}
pub type node_list = _node_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _node_list {
    pub node: *mut Agnode_t,
    pub next: *mut _node_list,
}
pub type block_t = _block;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block {
    pub mem: *mut cell,
    pub cur: *mut cell,
    pub endp: *mut cell,
    pub next: *mut _block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cell {
    pub p: gridpt,
    pub nodes: *mut node_list,
    pub link: Dtlink_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gridpt {
    pub i: libc::c_int,
    pub j: libc::c_int,
}
pub type Grid = _grid;
pub type walkfn_t = Option::<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut libc::c_void) -> libc::c_int,
>;
unsafe extern "C" fn newBlock(mut size: libc::c_int) -> *mut block_t {
    let mut newb: *mut block_t = 0 as *mut block_t;
    newb = gmalloc(::std::mem::size_of::<block_t>() as libc::c_ulong) as *mut block_t;
    let ref mut fresh0 = (*newb).next;
    *fresh0 = 0 as *mut _block;
    let ref mut fresh1 = (*newb).mem;
    *fresh1 = gcalloc(size as size_t, ::std::mem::size_of::<cell>() as libc::c_ulong)
        as *mut cell;
    let ref mut fresh2 = (*newb).endp;
    *fresh2 = ((*newb).mem).offset(size as isize);
    let ref mut fresh3 = (*newb).cur;
    *fresh3 = (*newb).mem;
    return newb;
}
unsafe extern "C" fn freeBlock(mut b: *mut block_t) {
    if !b.is_null() {
        let mut next: *mut block_t = (*b).next;
        free((*b).mem as *mut libc::c_void);
        free(b as *mut libc::c_void);
        freeBlock(next);
    }
}
unsafe extern "C" fn getCell(mut g: *mut Grid) -> *mut cell {
    let mut cp: *mut cell = 0 as *mut cell;
    let mut bp: *mut block_t = (*g).cellCur;
    if (*bp).cur == (*bp).endp {
        if ((*bp).next).is_null() {
            let ref mut fresh4 = (*bp).next;
            *fresh4 = newBlock(
                (2 as libc::c_int as libc::c_long
                    * ((*bp).endp).offset_from((*bp).mem) as libc::c_long) as libc::c_int,
            );
        }
        let ref mut fresh5 = (*g).cellCur;
        *fresh5 = (*bp).next;
        bp = *fresh5;
        let ref mut fresh6 = (*bp).cur;
        *fresh6 = (*bp).mem;
    }
    let ref mut fresh7 = (*bp).cur;
    let fresh8 = *fresh7;
    *fresh7 = (*fresh7).offset(1);
    cp = fresh8;
    return cp;
}
unsafe extern "C" fn ijcmpf(
    mut d: *mut Dt_t,
    mut p1: *mut gridpt,
    mut p2: *mut gridpt,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    let mut diff: libc::c_int = 0;
    diff = (*p1).i - (*p2).i;
    if diff != 0 { return diff } else { return (*p1).j - (*p2).j };
}
static mut _grid: *mut Grid = 0 as *const Grid as *mut Grid;
unsafe extern "C" fn newCell(
    mut d: *mut Dt_t,
    mut obj: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) -> *mut libc::c_void {
    let mut cellp: *mut cell = obj as *mut cell;
    let mut newp: *mut cell = 0 as *mut cell;
    newp = getCell(_grid);
    (*newp).p.i = (*cellp).p.i;
    (*newp).p.j = (*cellp).p.j;
    let ref mut fresh9 = (*newp).nodes;
    *fresh9 = 0 as *mut node_list;
    return newp as *mut libc::c_void;
}
unsafe extern "C" fn newNode(
    mut g: *mut Grid,
    mut n: *mut Agnode_t,
    mut nxt: *mut node_list,
) -> *mut node_list {
    let mut newp: *mut node_list = 0 as *mut node_list;
    let ref mut fresh10 = (*g).listCur;
    let fresh11 = *fresh10;
    *fresh10 = (*fresh10).offset(1);
    newp = fresh11;
    let ref mut fresh12 = (*newp).node;
    *fresh12 = n;
    let ref mut fresh13 = (*newp).next;
    *fresh13 = nxt;
    return newp;
}
static mut gridDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 0 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<gridpt>() as libc::c_ulong as libc::c_int,
            link: 16 as libc::c_ulong as libc::c_int,
            makef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        *mut Dtdisc_t,
                    ) -> *mut libc::c_void,
                >,
                Dtmake_f,
            >(
                Some(
                    newCell
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut libc::c_void,
                            *mut Dtdisc_t,
                        ) -> *mut libc::c_void,
                ),
            ),
            freef: None,
            comparf: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut gridpt,
                        *mut gridpt,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
                >,
                Dtcompar_f,
            >(
                Some(
                    ijcmpf
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut gridpt,
                            *mut gridpt,
                            *mut Dtdisc_t,
                        ) -> libc::c_int,
                ),
            ),
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn mkGrid(mut cellHint: libc::c_int) -> *mut Grid {
    let mut g: *mut Grid = 0 as *mut Grid;
    g = gmalloc(::std::mem::size_of::<Grid>() as libc::c_ulong) as *mut Grid;
    _grid = g;
    let ref mut fresh14 = (*g).data;
    *fresh14 = dtopen(&mut gridDisc, Dtoset);
    let ref mut fresh15 = (*g).listMem;
    *fresh15 = 0 as *mut node_list;
    (*g).listSize = 0 as libc::c_int;
    let ref mut fresh16 = (*g).cellMem;
    *fresh16 = newBlock(cellHint);
    return g;
}
#[no_mangle]
pub unsafe extern "C" fn adjustGrid(mut g: *mut Grid, mut nnodes: libc::c_int) {
    let mut nsize: libc::c_int = 0;
    if nnodes > (*g).listSize {
        nsize = if nnodes > 2 as libc::c_int * (*g).listSize {
            nnodes
        } else {
            2 as libc::c_int * (*g).listSize
        };
        if !((*g).listMem).is_null() {
            free((*g).listMem as *mut libc::c_void);
        }
        let ref mut fresh17 = (*g).listMem;
        *fresh17 = gcalloc(
            nsize as size_t,
            ::std::mem::size_of::<node_list>() as libc::c_ulong,
        ) as *mut node_list;
        (*g).listSize = nsize;
    }
}
#[no_mangle]
pub unsafe extern "C" fn clearGrid(mut g: *mut Grid) {
    (Some(((*(*g).data).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*g).data, 0 as *mut libc::c_void, 0o100 as libc::c_int);
    let ref mut fresh18 = (*g).listCur;
    *fresh18 = (*g).listMem;
    let ref mut fresh19 = (*g).cellCur;
    *fresh19 = (*g).cellMem;
    let ref mut fresh20 = (*(*g).cellCur).cur;
    *fresh20 = (*(*g).cellCur).mem;
}
#[no_mangle]
pub unsafe extern "C" fn delGrid(mut g: *mut Grid) {
    dtclose((*g).data);
    freeBlock((*g).cellMem);
    free((*g).listMem as *mut libc::c_void);
    free(g as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn addGrid(
    mut g: *mut Grid,
    mut i: libc::c_int,
    mut j: libc::c_int,
    mut n: *mut Agnode_t,
) {
    let mut cellp: *mut cell = 0 as *mut cell;
    let mut key: cell = cell {
        p: gridpt { i: 0, j: 0 },
        nodes: 0 as *mut node_list,
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
    };
    key.p.i = i;
    key.p.j = j;
    cellp = (Some(((*(*g).data).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*g).data, &mut key as *mut cell as *mut libc::c_void, 0o1 as libc::c_int)
        as *mut cell;
    let ref mut fresh21 = (*cellp).nodes;
    *fresh21 = newNode(g, n, (*cellp).nodes);
    if Verbose as libc::c_int >= 3 as libc::c_int {
        fprintf(
            stderr,
            b"grid(%d,%d): %s\n\0" as *const u8 as *const libc::c_char,
            i,
            j,
            agnameof(n as *mut libc::c_void),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn walkGrid(
    mut g: *mut Grid,
    mut walkf: Option::<
        unsafe extern "C" fn(*mut Dt_t, *mut cell, *mut Grid) -> libc::c_int,
    >,
) {
    dtwalk(
        (*g).data,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut Dt_t, *mut cell, *mut Grid) -> libc::c_int,
            >,
            walkfn_t,
        >(walkf),
        g as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn findGrid(
    mut g: *mut Grid,
    mut i: libc::c_int,
    mut j: libc::c_int,
) -> *mut cell {
    let mut key: cell = cell {
        p: gridpt { i: 0, j: 0 },
        nodes: 0 as *mut node_list,
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
    };
    key.p.i = i;
    key.p.j = j;
    return (Some(((*(*g).data).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*g).data, &mut key as *mut cell as *mut libc::c_void, 0o4 as libc::c_int)
        as *mut cell;
}
#[no_mangle]
pub unsafe extern "C" fn gLength(mut p: *mut cell) -> libc::c_int {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut nodes: *mut node_list = (*p).nodes;
    while !nodes.is_null() {
        len += 1;
        nodes = (*nodes).next;
    }
    return len;
}
