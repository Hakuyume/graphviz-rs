#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut Nop: libc::c_int;
    fn neato_layout(g: *mut graph_t);
    fn fdp_layout(g: *mut graph_t);
    fn sfdp_layout(g: *mut graph_t);
    fn twopi_layout(g: *mut graph_t);
    fn circo_layout(g: *mut graph_t);
    fn patchwork_layout(g: *mut graph_t);
    fn osage_layout(g: *mut graph_t);
    fn neato_cleanup(g: *mut graph_t);
    fn fdp_cleanup(g: *mut graph_t);
    fn sfdp_cleanup(g: *mut graph_t);
    fn twopi_cleanup(g: *mut graph_t);
    fn circo_cleanup(g: *mut graph_t);
    fn patchwork_cleanup(g: *mut graph_t);
    fn osage_cleanup(g: *mut graph_t);
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
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
pub type uint64_t = __uint64_t;
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
pub type graph_t = Agraph_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvlayout_features_t {
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_installed_t {
    pub id: libc::c_int,
    pub type_0: *const libc::c_char,
    pub quality: libc::c_int,
    pub engine: *mut libc::c_void,
    pub features: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvlayout_engine_s {
    pub layout: Option::<unsafe extern "C" fn(*mut graph_t) -> ()>,
    pub cleanup: Option::<unsafe extern "C" fn(*mut graph_t) -> ()>,
}
pub type gvlayout_engine_t = gvlayout_engine_s;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const LAYOUT_NOP2: C2RustUnnamed_2 = 8;
pub const LAYOUT_NOP1: C2RustUnnamed_2 = 7;
pub const LAYOUT_CLUSTER: C2RustUnnamed_2 = 6;
pub const LAYOUT_PATCHWORK: C2RustUnnamed_2 = 5;
pub const LAYOUT_CIRCO: C2RustUnnamed_2 = 4;
pub const LAYOUT_TWOPI: C2RustUnnamed_2 = 3;
pub const LAYOUT_SFDP: C2RustUnnamed_2 = 2;
pub const LAYOUT_FDP: C2RustUnnamed_2 = 1;
pub const LAYOUT_NEATO: C2RustUnnamed_2 = 0;
unsafe extern "C" fn nop1_layout(mut g: *mut graph_t) {
    Nop = 1 as libc::c_int;
    neato_layout(g);
    Nop = 0 as libc::c_int;
}
unsafe extern "C" fn nop2_layout(mut g: *mut graph_t) {
    Nop = 2 as libc::c_int;
    neato_layout(g);
    Nop = 0 as libc::c_int;
}
#[no_mangle]
pub static mut neatogen_engine: gvlayout_engine_t = unsafe {
    {
        let mut init = gvlayout_engine_s {
            layout: Some(neato_layout as unsafe extern "C" fn(*mut graph_t) -> ()),
            cleanup: Some(neato_cleanup as unsafe extern "C" fn(*mut graph_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut fdpgen_engine: gvlayout_engine_t = unsafe {
    {
        let mut init = gvlayout_engine_s {
            layout: Some(fdp_layout as unsafe extern "C" fn(*mut graph_t) -> ()),
            cleanup: Some(fdp_cleanup as unsafe extern "C" fn(*mut graph_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut sfdpgen_engine: gvlayout_engine_t = unsafe {
    {
        let mut init = gvlayout_engine_s {
            layout: Some(sfdp_layout as unsafe extern "C" fn(*mut graph_t) -> ()),
            cleanup: Some(sfdp_cleanup as unsafe extern "C" fn(*mut graph_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut twopigen_engine: gvlayout_engine_t = unsafe {
    {
        let mut init = gvlayout_engine_s {
            layout: Some(twopi_layout as unsafe extern "C" fn(*mut graph_t) -> ()),
            cleanup: Some(twopi_cleanup as unsafe extern "C" fn(*mut graph_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut circogen_engine: gvlayout_engine_t = unsafe {
    {
        let mut init = gvlayout_engine_s {
            layout: Some(circo_layout as unsafe extern "C" fn(*mut graph_t) -> ()),
            cleanup: Some(circo_cleanup as unsafe extern "C" fn(*mut graph_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut nop1gen_engine: gvlayout_engine_t = unsafe {
    {
        let mut init = gvlayout_engine_s {
            layout: Some(nop1_layout as unsafe extern "C" fn(*mut graph_t) -> ()),
            cleanup: Some(neato_cleanup as unsafe extern "C" fn(*mut graph_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut nop2gen_engine: gvlayout_engine_t = unsafe {
    {
        let mut init = gvlayout_engine_s {
            layout: Some(nop2_layout as unsafe extern "C" fn(*mut graph_t) -> ()),
            cleanup: Some(neato_cleanup as unsafe extern "C" fn(*mut graph_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut patchwork_engine: gvlayout_engine_t = unsafe {
    {
        let mut init = gvlayout_engine_s {
            layout: Some(patchwork_layout as unsafe extern "C" fn(*mut graph_t) -> ()),
            cleanup: Some(patchwork_cleanup as unsafe extern "C" fn(*mut graph_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut osage_engine: gvlayout_engine_t = unsafe {
    {
        let mut init = gvlayout_engine_s {
            layout: Some(osage_layout as unsafe extern "C" fn(*mut graph_t) -> ()),
            cleanup: Some(osage_cleanup as unsafe extern "C" fn(*mut graph_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut neatogen_features: gvlayout_features_t = {
    let mut init = gvlayout_features_t {
        flags: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
pub static mut gvlayout_neato_types: [gvplugin_installed_t; 11] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: LAYOUT_NEATO as libc::c_int,
                type_0: b"neato\0" as *const u8 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: &neatogen_engine as *const gvlayout_engine_t
                    as *mut gvlayout_engine_t as *mut libc::c_void,
                features: &neatogen_features as *const gvlayout_features_t
                    as *mut gvlayout_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: LAYOUT_FDP as libc::c_int,
                type_0: b"fdp\0" as *const u8 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: &fdpgen_engine as *const gvlayout_engine_t
                    as *mut gvlayout_engine_t as *mut libc::c_void,
                features: &neatogen_features as *const gvlayout_features_t
                    as *mut gvlayout_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: LAYOUT_SFDP as libc::c_int,
                type_0: b"sfdp\0" as *const u8 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: &sfdpgen_engine as *const gvlayout_engine_t
                    as *mut gvlayout_engine_t as *mut libc::c_void,
                features: &neatogen_features as *const gvlayout_features_t
                    as *mut gvlayout_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: LAYOUT_TWOPI as libc::c_int,
                type_0: b"twopi\0" as *const u8 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: &twopigen_engine as *const gvlayout_engine_t
                    as *mut gvlayout_engine_t as *mut libc::c_void,
                features: &neatogen_features as *const gvlayout_features_t
                    as *mut gvlayout_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: LAYOUT_CIRCO as libc::c_int,
                type_0: b"circo\0" as *const u8 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: &circogen_engine as *const gvlayout_engine_t
                    as *mut gvlayout_engine_t as *mut libc::c_void,
                features: &neatogen_features as *const gvlayout_features_t
                    as *mut gvlayout_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: LAYOUT_PATCHWORK as libc::c_int,
                type_0: b"patchwork\0" as *const u8 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: &patchwork_engine as *const gvlayout_engine_t
                    as *mut gvlayout_engine_t as *mut libc::c_void,
                features: &neatogen_features as *const gvlayout_features_t
                    as *mut gvlayout_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: LAYOUT_CLUSTER as libc::c_int,
                type_0: b"osage\0" as *const u8 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: &osage_engine as *const gvlayout_engine_t
                    as *mut gvlayout_engine_t as *mut libc::c_void,
                features: &neatogen_features as *const gvlayout_features_t
                    as *mut gvlayout_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: LAYOUT_NOP1 as libc::c_int,
                type_0: b"nop\0" as *const u8 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: &nop1gen_engine as *const gvlayout_engine_t
                    as *mut gvlayout_engine_t as *mut libc::c_void,
                features: &neatogen_features as *const gvlayout_features_t
                    as *mut gvlayout_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: LAYOUT_NOP1 as libc::c_int,
                type_0: b"nop1\0" as *const u8 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: &nop1gen_engine as *const gvlayout_engine_t
                    as *mut gvlayout_engine_t as *mut libc::c_void,
                features: &neatogen_features as *const gvlayout_features_t
                    as *mut gvlayout_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: LAYOUT_NOP1 as libc::c_int,
                type_0: b"nop2\0" as *const u8 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: &nop2gen_engine as *const gvlayout_engine_t
                    as *mut gvlayout_engine_t as *mut libc::c_void,
                features: &neatogen_features as *const gvlayout_features_t
                    as *mut gvlayout_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: 0 as libc::c_int,
                type_0: 0 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
    ]
};
