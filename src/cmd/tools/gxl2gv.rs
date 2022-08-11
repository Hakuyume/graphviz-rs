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
#![feature(c_variadic, extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type XML_ParserStruct;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut Dtoset: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    static mut AgDefaultDisc: Agdisc_t;
    fn agopen(name: *mut libc::c_char, desc: Agdesc_t, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agnode(g: *mut Agraph_t, name: *mut libc::c_char, createflag: libc::c_int) -> *mut Agnode_t;
    fn agedge(
        g: *mut Agraph_t,
        t: *mut Agnode_t,
        h: *mut Agnode_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agdelete(g: *mut Agraph_t, obj: *mut libc::c_void) -> libc::c_int;
    fn agstrdup_html(_: *mut Agraph_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn agstrfree(_: *mut Agraph_t, _: *const libc::c_char) -> libc::c_int;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agxset(obj: *mut libc::c_void, sym: *mut Agsym_t, value: *const libc::c_char)
        -> libc::c_int;
    fn agsubg(g: *mut Agraph_t, name: *mut libc::c_char, cflag: libc::c_int) -> *mut Agraph_t;
    static mut Agdirected: Agdesc_t;
    static mut Agundirected: Agdesc_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn agrename(obj: *mut Agobj_t, newname: *mut libc::c_char) -> libc::c_int;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn XML_ParserCreate(encoding: *const XML_Char) -> XML_Parser;
    fn XML_SetElementHandler(
        parser: XML_Parser,
        start: XML_StartElementHandler,
        end: XML_EndElementHandler,
    );
    fn XML_SetCharacterDataHandler(parser: XML_Parser, handler: XML_CharacterDataHandler);
    fn XML_SetUserData(parser: XML_Parser, userData: *mut libc::c_void);
    fn XML_Parse(
        parser: XML_Parser,
        s: *const libc::c_char,
        len: libc::c_int,
        isFinal: libc::c_int,
    ) -> XML_Status;
    fn XML_GetErrorCode(parser: XML_Parser) -> XML_Error;
    fn XML_GetCurrentLineNumber(parser: XML_Parser) -> XML_Size;
    fn XML_ErrorString(code: XML_Error) -> *const XML_LChar;
    fn XML_ParserFree(parser: XML_Parser);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_2 = 8;
pub const _ISpunct: C2RustUnnamed_2 = 4;
pub const _IScntrl: C2RustUnnamed_2 = 2;
pub const _ISblank: C2RustUnnamed_2 = 1;
pub const _ISgraph: C2RustUnnamed_2 = 32768;
pub const _ISprint: C2RustUnnamed_2 = 16384;
pub const _ISspace: C2RustUnnamed_2 = 8192;
pub const _ISxdigit: C2RustUnnamed_2 = 4096;
pub const _ISdigit: C2RustUnnamed_2 = 2048;
pub const _ISalpha: C2RustUnnamed_2 = 1024;
pub const _ISlower: C2RustUnnamed_2 = 512;
pub const _ISupper: C2RustUnnamed_2 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gv_stack_t {
    pub base: *mut *mut libc::c_void,
    pub size: size_t,
    pub capacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct userdata_t {
    pub xml_attr_name: agxbuf,
    pub xml_attr_value: agxbuf,
    pub composite_buffer: agxbuf,
    pub elements: gv_stack_t,
    pub listen: libc::c_int,
    pub closedElementType: attr_t,
    pub globalAttrType: attr_t,
    pub compositeReadState: libc::c_int,
    pub edgeinverted: libc::c_int,
    pub nameMap: *mut Dt_t,
}
pub type attr_t = libc::c_uint;
pub const TAG_HTML_LIKE_STRING: attr_t = 4;
pub const TAG_EDGE: attr_t = 3;
pub const TAG_NODE: attr_t = 2;
pub const TAG_GRAPH: attr_t = 1;
pub const TAG_NONE: attr_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct agxbuf {
    pub buf: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub eptr: *mut libc::c_char,
    pub dyna: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct namev_t {
    pub link: Dtlink_t,
    pub name: *mut libc::c_char,
    pub unique_name: *mut libc::c_char,
}
pub type XML_Parser = *mut XML_ParserStruct;
pub type XML_Char = libc::c_char;
pub type XML_Size = libc::c_ulong;
pub type XML_LChar = libc::c_char;
pub type XML_Error = libc::c_uint;
pub const XML_ERROR_AMPLIFICATION_LIMIT_BREACH: XML_Error = 43;
pub const XML_ERROR_NO_BUFFER: XML_Error = 42;
pub const XML_ERROR_INVALID_ARGUMENT: XML_Error = 41;
pub const XML_ERROR_RESERVED_NAMESPACE_URI: XML_Error = 40;
pub const XML_ERROR_RESERVED_PREFIX_XMLNS: XML_Error = 39;
pub const XML_ERROR_RESERVED_PREFIX_XML: XML_Error = 38;
pub const XML_ERROR_SUSPEND_PE: XML_Error = 37;
pub const XML_ERROR_FINISHED: XML_Error = 36;
pub const XML_ERROR_ABORTED: XML_Error = 35;
pub const XML_ERROR_NOT_SUSPENDED: XML_Error = 34;
pub const XML_ERROR_SUSPENDED: XML_Error = 33;
pub const XML_ERROR_PUBLICID: XML_Error = 32;
pub const XML_ERROR_TEXT_DECL: XML_Error = 31;
pub const XML_ERROR_XML_DECL: XML_Error = 30;
pub const XML_ERROR_INCOMPLETE_PE: XML_Error = 29;
pub const XML_ERROR_UNDECLARING_PREFIX: XML_Error = 28;
pub const XML_ERROR_UNBOUND_PREFIX: XML_Error = 27;
pub const XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING: XML_Error = 26;
pub const XML_ERROR_FEATURE_REQUIRES_XML_DTD: XML_Error = 25;
pub const XML_ERROR_ENTITY_DECLARED_IN_PE: XML_Error = 24;
pub const XML_ERROR_UNEXPECTED_STATE: XML_Error = 23;
pub const XML_ERROR_NOT_STANDALONE: XML_Error = 22;
pub const XML_ERROR_EXTERNAL_ENTITY_HANDLING: XML_Error = 21;
pub const XML_ERROR_UNCLOSED_CDATA_SECTION: XML_Error = 20;
pub const XML_ERROR_INCORRECT_ENCODING: XML_Error = 19;
pub const XML_ERROR_UNKNOWN_ENCODING: XML_Error = 18;
pub const XML_ERROR_MISPLACED_XML_PI: XML_Error = 17;
pub const XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF: XML_Error = 16;
pub const XML_ERROR_BINARY_ENTITY_REF: XML_Error = 15;
pub const XML_ERROR_BAD_CHAR_REF: XML_Error = 14;
pub const XML_ERROR_ASYNC_ENTITY: XML_Error = 13;
pub const XML_ERROR_RECURSIVE_ENTITY_REF: XML_Error = 12;
pub const XML_ERROR_UNDEFINED_ENTITY: XML_Error = 11;
pub const XML_ERROR_PARAM_ENTITY_REF: XML_Error = 10;
pub const XML_ERROR_JUNK_AFTER_DOC_ELEMENT: XML_Error = 9;
pub const XML_ERROR_DUPLICATE_ATTRIBUTE: XML_Error = 8;
pub const XML_ERROR_TAG_MISMATCH: XML_Error = 7;
pub const XML_ERROR_PARTIAL_CHAR: XML_Error = 6;
pub const XML_ERROR_UNCLOSED_TOKEN: XML_Error = 5;
pub const XML_ERROR_INVALID_TOKEN: XML_Error = 4;
pub const XML_ERROR_NO_ELEMENTS: XML_Error = 3;
pub const XML_ERROR_SYNTAX: XML_Error = 2;
pub const XML_ERROR_NO_MEMORY: XML_Error = 1;
pub const XML_ERROR_NONE: XML_Error = 0;
pub const XML_STATUS_ERROR: XML_Status = 0;
pub type XML_Status = libc::c_uint;
pub const XML_STATUS_SUSPENDED: XML_Status = 2;
pub const XML_STATUS_OK: XML_Status = 1;
pub type XML_CharacterDataHandler =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const XML_Char, libc::c_int) -> ()>;
pub const FIRST_ALLOCATION: C2RustUnnamed_3 = 512;
pub type C2RustUnnamed_3 = libc::c_uint;
pub type XML_EndElementHandler =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const XML_Char) -> ()>;
pub type XML_StartElementHandler =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const XML_Char, *mut *const XML_Char) -> ()>;
#[inline]
unsafe extern "C" fn gv_calloc(mut nmemb: size_t, mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = calloc(nmemb, size);
    if (nmemb > 0 as libc::c_int as libc::c_ulong
        && size > 0 as libc::c_int as libc::c_ulong
        && p.is_null()) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return p;
}
#[inline]
unsafe extern "C" fn gv_realloc(
    mut ptr: *mut libc::c_void,
    mut old_size: size_t,
    mut new_size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = realloc(ptr, new_size);
    if (new_size > 0 as libc::c_int as libc::c_ulong && p.is_null()) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    if new_size > old_size {
        memset(
            (p as *mut libc::c_char).offset(old_size as isize) as *mut libc::c_void,
            0 as libc::c_int,
            new_size.wrapping_sub(old_size),
        );
    }
    return p;
}
#[inline]
unsafe extern "C" fn gv_recalloc(
    mut ptr: *mut libc::c_void,
    mut old_nmemb: size_t,
    mut new_nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    if size > 0 as libc::c_int as libc::c_ulong
        && !(b"attempt to allocate array of 0-sized elements\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"size > 0 && \"attempt to allocate array of 0-sized elements\"\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void *gv_recalloc(void *, size_t, size_t, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    if old_nmemb < (18446744073709551615 as libc::c_ulong).wrapping_div(size)
        && !(b"claimed previous extent is too large\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"old_nmemb < SIZE_MAX / size && \"claimed previous extent is too large\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void *gv_recalloc(void *, size_t, size_t, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    if (new_nmemb > (18446744073709551615 as libc::c_ulong).wrapping_div(size)) as libc::c_int
        as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"integer overflow in dynamic memory reallocation\n\0" as *const u8
                as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return gv_realloc(
        ptr,
        old_nmemb.wrapping_mul(size),
        new_nmemb.wrapping_mul(size),
    );
}
#[inline]
unsafe extern "C" fn gv_strdup(mut original: *const libc::c_char) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = strdup(original);
    if (copy == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return copy;
}
#[inline]
unsafe extern "C" fn agxbprint(
    mut xb: *mut agxbuf,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut size: size_t = 0;
    let mut result: libc::c_int = 0;
    ap = args.clone();
    let mut ap2: ::std::ffi::VaListImpl;
    let mut rc: libc::c_int = 0;
    ap2 = ap.clone();
    rc = vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        fmt,
        ap2.as_va_list(),
    );
    if rc < 0 as libc::c_int {
        return rc;
    }
    size = (rc as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut unused_space: size_t = ((*xb).eptr).offset_from((*xb).ptr) as libc::c_long as size_t;
    if unused_space < size {
        let mut extra: size_t = size.wrapping_sub(unused_space);
        agxbmore(xb, extra);
    }
    result = vsnprintf((*xb).ptr, size, fmt, ap.as_va_list());
    if result == size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
        || result < 0 as libc::c_int
    {
    } else {
        __assert_fail(
            b"result == (int)(size - 1) || result < 0\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/agxbuf.h\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                b"int agxbprint(agxbuf *, const char *, ...)\0",
            ))
            .as_ptr(),
        );
    }
    if result > 0 as libc::c_int {
        let ref mut fresh0 = (*xb).ptr;
        *fresh0 = (*fresh0).offset(result as size_t as isize);
    }
    return result;
}
#[inline]
unsafe extern "C" fn agxbfree(mut xb: *mut agxbuf) {
    if (*xb).dyna != 0 {
        free((*xb).buf as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn agxbmore(mut xb: *mut agxbuf, mut ssz: size_t) {
    let mut cnt: size_t = 0 as libc::c_int as size_t;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut nsize: size_t = 0 as libc::c_int as size_t;
    let mut nbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    size = ((*xb).eptr).offset_from((*xb).buf) as libc::c_long as size_t;
    nsize = (2 as libc::c_int as libc::c_ulong).wrapping_mul(size);
    if size.wrapping_add(ssz) > nsize {
        nsize = size.wrapping_add(ssz);
    }
    cnt = ((*xb).ptr).offset_from((*xb).buf) as libc::c_long as size_t;
    if (*xb).dyna != 0 {
        nbuf = gv_recalloc(
            (*xb).buf as *mut libc::c_void,
            size,
            nsize,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    } else {
        nbuf = gv_calloc(
            nsize,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
        memcpy(
            nbuf as *mut libc::c_void,
            (*xb).buf as *const libc::c_void,
            cnt,
        );
        (*xb).dyna = 1 as libc::c_int;
    }
    let ref mut fresh1 = (*xb).buf;
    *fresh1 = nbuf;
    let ref mut fresh2 = (*xb).ptr;
    *fresh2 = ((*xb).buf).offset(cnt as isize);
    let ref mut fresh3 = (*xb).eptr;
    *fresh3 = ((*xb).buf).offset(nsize as isize);
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn agxbinit(
    mut xb: *mut agxbuf,
    mut hint: libc::c_uint,
    mut init: *mut libc::c_char,
) {
    if !init.is_null() {
        let ref mut fresh4 = (*xb).buf;
        *fresh4 = init;
        (*xb).dyna = 0 as libc::c_int;
    } else {
        if hint == 0 as libc::c_int as libc::c_uint {
            hint = 8192 as libc::c_int as libc::c_uint;
        }
        (*xb).dyna = 1 as libc::c_int;
        let ref mut fresh5 = (*xb).buf;
        *fresh5 = gv_calloc(
            hint as size_t,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    }
    let ref mut fresh6 = (*xb).eptr;
    *fresh6 = ((*xb).buf).offset(hint as isize);
    let ref mut fresh7 = (*xb).ptr;
    *fresh7 = (*xb).buf;
    *(*xb).ptr = '\0' as i32 as libc::c_char;
}
#[inline]
unsafe extern "C" fn agxbput_n(
    mut xb: *mut agxbuf,
    mut s: *const libc::c_char,
    mut ssz: size_t,
) -> size_t {
    if ((*xb).ptr).offset(ssz as isize) > (*xb).eptr {
        agxbmore(xb, ssz);
    }
    memcpy(
        (*xb).ptr as *mut libc::c_void,
        s as *const libc::c_void,
        ssz,
    );
    let ref mut fresh8 = (*xb).ptr;
    *fresh8 = (*fresh8).offset(ssz as isize);
    return ssz;
}
#[inline]
unsafe extern "C" fn agxbput(mut xb: *mut agxbuf, mut s: *const libc::c_char) -> size_t {
    let mut ssz: size_t = strlen(s);
    return agxbput_n(xb, s, ssz);
}
#[inline]
unsafe extern "C" fn agxbputc(mut xb: *mut agxbuf, mut c: libc::c_char) -> libc::c_int {
    if (*xb).ptr >= (*xb).eptr {
        agxbmore(xb, 1 as libc::c_int as size_t);
    }
    let ref mut fresh9 = (*xb).ptr;
    let fresh10 = *fresh9;
    *fresh9 = (*fresh9).offset(1);
    *fresh10 = c;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn agxbuse(mut xb: *mut agxbuf) -> *mut libc::c_char {
    agxbputc(xb, '\0' as i32 as libc::c_char);
    let ref mut fresh11 = (*xb).ptr;
    *fresh11 = (*xb).buf;
    return (*xb).ptr;
}
#[inline]
unsafe extern "C" fn agxblen(mut xb: *const agxbuf) -> size_t {
    return ((*xb).ptr).offset_from((*xb).buf) as libc::c_long as size_t;
}
#[inline]
unsafe extern "C" fn agxbclear(mut xb: *mut agxbuf) {
    let ref mut fresh12 = (*xb).ptr;
    *fresh12 = (*xb).buf;
}
#[inline]
unsafe extern "C" fn stack_size(mut stack: *const gv_stack_t) -> size_t {
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            23 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"size_t stack_size(const gv_stack_t *)\0",
            ))
            .as_ptr(),
        );
    }
    return (*stack).size;
}
#[inline]
unsafe extern "C" fn stack_is_empty(mut stack: *const gv_stack_t) -> bool {
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"_Bool stack_is_empty(const gv_stack_t *)\0",
            ))
            .as_ptr(),
        );
    }
    return stack_size(stack) == 0 as libc::c_int as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn stack_push(
    mut stack: *mut gv_stack_t,
    mut item: *mut libc::c_void,
) -> libc::c_int {
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"int stack_push(gv_stack_t *, void *)\0",
            ))
            .as_ptr(),
        );
    }
    if (*stack).size == (*stack).capacity {
        if ((18446744073709551615 as libc::c_ulong).wrapping_div(2 as libc::c_int as libc::c_ulong)
            < (*stack).capacity) as libc::c_int as libc::c_long
            != 0
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
        if (b == 0 as *mut libc::c_void as *mut *mut libc::c_void) as libc::c_int as libc::c_long
            != 0
        {
            return 12 as libc::c_int;
        }
        (*stack).capacity = c;
        let ref mut fresh13 = (*stack).base;
        *fresh13 = b;
    }
    if !((*stack).base).is_null() {
    } else {
        __assert_fail(
            b"stack->base != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"int stack_push(gv_stack_t *, void *)\0",
            ))
            .as_ptr(),
        );
    }
    if (*stack).capacity > (*stack).size {
    } else {
        __assert_fail(
            b"stack->capacity > stack->size\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"int stack_push(gv_stack_t *, void *)\0",
            ))
            .as_ptr(),
        );
    }
    let ref mut fresh14 = *((*stack).base).offset((*stack).size as isize);
    *fresh14 = item;
    let ref mut fresh15 = (*stack).size;
    *fresh15 = (*fresh15).wrapping_add(1);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn stack_push_or_exit(mut stack: *mut gv_stack_t, mut item: *mut libc::c_void) {
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                b"void stack_push_or_exit(gv_stack_t *, void *)\0",
            ))
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
unsafe extern "C" fn stack_top(mut stack: *mut gv_stack_t) -> *mut libc::c_void {
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"void *stack_top(gv_stack_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if !stack_is_empty(stack)
        && !(b"access to top of an empty stack\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"!stack_is_empty(stack) && \"access to top of an empty stack\"\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"void *stack_top(gv_stack_t *)\0",
            ))
            .as_ptr(),
        );
    }
    return *((*stack).base)
        .offset(((*stack).size).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
}
#[inline]
unsafe extern "C" fn stack_pop(mut stack: *mut gv_stack_t) -> *mut libc::c_void {
    let mut top: *mut libc::c_void = stack_top(stack);
    let ref mut fresh16 = (*stack).size;
    *fresh16 = (*fresh16).wrapping_sub(1);
    return top;
}
#[inline]
unsafe extern "C" fn stack_reset(mut stack: *mut gv_stack_t) {
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                b"void stack_reset(gv_stack_t *)\0",
            ))
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
unsafe extern "C" fn pushString(mut stk: *mut gv_stack_t, mut s: *const libc::c_char) {
    let mut copy: *mut libc::c_char = gv_strdup(s);
    stack_push_or_exit(stk, copy as *mut libc::c_void);
}
unsafe extern "C" fn popString(mut stk: *mut gv_stack_t) {
    if stack_is_empty(stk) as libc::c_long != 0 {
        fprintf(
            stderr,
            b"PANIC: gxl2gv: empty element stack\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    let mut s: *mut libc::c_char = stack_pop(stk) as *mut libc::c_char;
    free(s as *mut libc::c_void);
}
unsafe extern "C" fn topString(mut stk: *mut gv_stack_t) -> *mut libc::c_char {
    if stack_is_empty(stk) as libc::c_long != 0 {
        fprintf(
            stderr,
            b"PANIC: gxl2gv: empty element stack\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return stack_top(stk) as *mut libc::c_char;
}
unsafe extern "C" fn freeString(mut stk: *mut gv_stack_t) {
    while !stack_is_empty(stk) {
        let mut s: *mut libc::c_char = stack_pop(stk) as *mut libc::c_char;
        free(s as *mut libc::c_void);
    }
    stack_reset(stk);
}
static mut root: *mut Agraph_t = 0 as *const Agraph_t as *mut Agraph_t;
static mut Current_class: attr_t = TAG_NONE;
static mut G: *mut Agraph_t = 0 as *const Agraph_t as *mut Agraph_t;
static mut N: *mut Agnode_t = 0 as *const Agnode_t as *mut Agnode_t;
static mut E: *mut Agedge_t = 0 as *const Agedge_t as *mut Agedge_t;
static mut Gstack: gv_stack_t = gv_stack_t {
    base: 0 as *const *mut libc::c_void as *mut *mut libc::c_void,
    size: 0,
    capacity: 0,
};
unsafe extern "C" fn make_nitem(
    mut d: *mut Dt_t,
    mut objp: *mut namev_t,
    mut disc: *mut Dtdisc_t,
) -> *mut namev_t {
    let mut np: *mut namev_t =
        malloc(::std::mem::size_of::<namev_t>() as libc::c_ulong) as *mut namev_t;
    if np.is_null() {
        return 0 as *mut namev_t;
    }
    let ref mut fresh17 = (*np).name;
    *fresh17 = (*objp).name;
    let ref mut fresh18 = (*np).unique_name;
    *fresh18 = 0 as *mut libc::c_char;
    return np;
}
unsafe extern "C" fn free_nitem(mut d: *mut Dt_t, mut np: *mut namev_t, mut disc: *mut Dtdisc_t) {
    free((*np).unique_name as *mut libc::c_void);
    free(np as *mut libc::c_void);
}
static mut nameDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: -(1 as libc::c_int),
            link: 0 as libc::c_ulong as libc::c_int,
            makef: ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut Dt_t, *mut namev_t, *mut Dtdisc_t) -> *mut namev_t,
                >,
                Dtmake_f,
            >(Some(
                make_nitem
                    as unsafe extern "C" fn(*mut Dt_t, *mut namev_t, *mut Dtdisc_t) -> *mut namev_t,
            )),
            freef: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(*mut Dt_t, *mut namev_t, *mut Dtdisc_t) -> ()>,
                Dtfree_f,
            >(Some(
                free_nitem as unsafe extern "C" fn(*mut Dt_t, *mut namev_t, *mut Dtdisc_t) -> (),
            )),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
unsafe extern "C" fn genUserdata() -> *mut userdata_t {
    let mut user: *mut userdata_t =
        malloc(::std::mem::size_of::<userdata_t>() as libc::c_ulong) as *mut userdata_t;
    if user.is_null() {
        return 0 as *mut userdata_t;
    }
    agxbinit(
        &mut (*user).xml_attr_name,
        100 as libc::c_int as libc::c_uint,
        0 as *mut libc::c_char,
    );
    agxbinit(
        &mut (*user).xml_attr_value,
        1000 as libc::c_int as libc::c_uint,
        0 as *mut libc::c_char,
    );
    agxbinit(
        &mut (*user).composite_buffer,
        1000 as libc::c_int as libc::c_uint,
        0 as *mut libc::c_char,
    );
    (*user).listen = 0 as libc::c_int;
    (*user).elements = {
        let mut init = gv_stack_t {
            base: 0 as *mut *mut libc::c_void,
            size: 0,
            capacity: 0,
        };
        init
    };
    (*user).closedElementType = TAG_NONE;
    (*user).globalAttrType = TAG_NONE;
    (*user).compositeReadState = 0 as libc::c_int;
    (*user).edgeinverted = 0 as libc::c_int;
    let ref mut fresh19 = (*user).nameMap;
    *fresh19 = dtopen(&mut nameDisc, Dtoset);
    return user;
}
unsafe extern "C" fn freeUserdata(mut ud: *mut userdata_t) {
    dtclose((*ud).nameMap);
    agxbfree(&mut (*ud).xml_attr_name);
    agxbfree(&mut (*ud).xml_attr_value);
    agxbfree(&mut (*ud).composite_buffer);
    freeString(&mut (*ud).elements);
    free(ud as *mut libc::c_void);
}
unsafe extern "C" fn addToMap(
    mut map: *mut Dt_t,
    mut name: *mut libc::c_char,
    mut uniqueName: *mut libc::c_char,
) {
    let mut obj: namev_t = namev_t {
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        name: 0 as *mut libc::c_char,
        unique_name: 0 as *mut libc::c_char,
    };
    let mut objp: *mut namev_t = 0 as *mut namev_t;
    obj.name = name;
    objp = (Some(((*map).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        map,
        &mut obj as *mut namev_t as *mut libc::c_void,
        0o1 as libc::c_int,
    ) as *mut namev_t;
    if ((*objp).unique_name).is_null() {
    } else {
        __assert_fail(
            b"objp->unique_name == 0\0" as *const u8 as *const libc::c_char,
            b"gxl2gv.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void addToMap(Dt_t *, char *, char *)\0",
            ))
            .as_ptr(),
        );
    }
    let ref mut fresh20 = (*objp).unique_name;
    *fresh20 = gv_strdup(uniqueName);
}
unsafe extern "C" fn mapLookup(
    mut nm: *mut Dt_t,
    mut name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut objp: *mut namev_t = (Some(((*nm).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        nm, name as *mut libc::c_void, 0o1000 as libc::c_int
    ) as *mut namev_t;
    if !objp.is_null() {
        return (*objp).unique_name;
    } else {
        return 0 as *mut libc::c_char;
    };
}
unsafe extern "C" fn isAnonGraph(mut name: *const libc::c_char) -> libc::c_int {
    let fresh21 = name;
    name = name.offset(1);
    if *fresh21 as libc::c_int != '%' as i32 {
        return 0 as libc::c_int;
    }
    while *(*__ctype_b_loc()).offset(*name as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        name = name.offset(1);
    }
    return (*name as libc::c_int == '\0' as i32) as libc::c_int;
}
unsafe extern "C" fn push_subg(mut g: *mut Agraph_t) {
    stack_push_or_exit(&mut Gstack, g as *mut libc::c_void);
    if stack_size(&mut Gstack) == 1 as libc::c_int as libc::c_ulong {
        root = g;
    }
    G = g;
}
unsafe extern "C" fn pop_subg() -> *mut Agraph_t {
    if stack_is_empty(&mut Gstack) {
        fprintf(
            stderr,
            b"gxl2gv: Gstack underflow in graph parser\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    let mut g: *mut Agraph_t = stack_pop(&mut Gstack) as *mut Agraph_t;
    if !stack_is_empty(&mut Gstack) {
        G = stack_top(&mut Gstack) as *mut Agraph_t;
    }
    return g;
}
unsafe extern "C" fn bind_node(mut name: *const libc::c_char) -> *mut Agnode_t {
    N = agnode(G, name as *mut libc::c_char, 1 as libc::c_int);
    return N;
}
unsafe extern "C" fn bind_edge(
    mut tail: *const libc::c_char,
    mut head: *const libc::c_char,
) -> *mut Agedge_t {
    let mut tailNode: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut headNode: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    tailNode = agnode(G, tail as *mut libc::c_char, 1 as libc::c_int);
    headNode = agnode(G, head as *mut libc::c_char, 1 as libc::c_int);
    E = agedge(G, tailNode, headNode, key, 1 as libc::c_int);
    return E;
}
unsafe extern "C" fn get_xml_attr(
    mut attrname: *mut libc::c_char,
    mut atts: *mut *const libc::c_char,
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    while !(*atts.offset(count as isize)).is_null() {
        if strcmp(attrname, *atts.offset(count as isize)) == 0 as libc::c_int {
            return count + 1 as libc::c_int;
        }
        count += 2 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn setName(
    mut names: *mut Dt_t,
    mut n: *mut Agobj_t,
    mut value: *mut libc::c_char,
) {
    let mut ap: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut oldName: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = agattr(
        root,
        ((*n).tag).objtype() as libc::c_int,
        b"_gxl_id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    agxset(n as *mut libc::c_void, ap, agnameof(n as *mut libc::c_void));
    oldName = agxget(n as *mut libc::c_void, ap);
    addToMap(names, oldName, value);
    agrename(n, value);
}
static mut defval: *mut libc::c_char =
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn setNodeAttr(
    mut np: *mut Agnode_t,
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut ud: *mut userdata_t,
    mut is_html: bool,
) {
    let mut ap: *mut Agsym_t = 0 as *mut Agsym_t;
    if strcmp(name, b"name\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        setName((*ud).nameMap, np as *mut Agobj_t, value);
    } else {
        ap = agattr(root, 1 as libc::c_int, name, 0 as *const libc::c_char);
        if ap.is_null() {
            ap = agattr(root, 1 as libc::c_int, name, defval);
        }
        if is_html {
            let mut val: *mut libc::c_char = agstrdup_html(root, value);
            agxset(np as *mut libc::c_void, ap, val);
            agstrfree(root, val);
        } else {
            agxset(np as *mut libc::c_void, ap, value);
        }
    };
}
unsafe extern "C" fn setGlobalNodeAttr(
    mut g: *mut Agraph_t,
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
) {
    if strncmp(
        name,
        b"node:\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) != 0
    {
        fprintf(
            stderr,
            b"Warning: global node attribute %s in graph %s does not begin with the prefix %s\n\0"
                as *const u8 as *const libc::c_char,
            name,
            agnameof(g as *mut libc::c_void),
            b"node:\0" as *const u8 as *const libc::c_char,
        );
    } else {
        name = name.offset(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
    }
    if g != root && (agattr(root, 1 as libc::c_int, name, 0 as *const libc::c_char)).is_null() {
        agattr(root, 1 as libc::c_int, name, defval);
    }
    agattr(G, 1 as libc::c_int, name, value);
}
unsafe extern "C" fn setEdgeAttr(
    mut ep: *mut Agedge_t,
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut ud: *mut userdata_t,
    mut is_html: bool,
) {
    let mut ap: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut attrname: *mut libc::c_char = 0 as *mut libc::c_char;
    if strcmp(name, b"headport\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if (*ud).edgeinverted != 0 {
            attrname = b"tailport\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            attrname = b"headport\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        ap = agattr(root, 2 as libc::c_int, attrname, 0 as *const libc::c_char);
        if ap.is_null() {
            ap = agattr(root, 2 as libc::c_int, attrname, defval);
        }
    } else if strcmp(name, b"tailport\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if (*ud).edgeinverted != 0 {
            attrname = b"headport\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            attrname = b"tailport\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        ap = agattr(root, 2 as libc::c_int, attrname, 0 as *const libc::c_char);
        if ap.is_null() {
            ap = agattr(root, 2 as libc::c_int, attrname, defval);
        }
    } else {
        ap = agattr(root, 2 as libc::c_int, name, 0 as *const libc::c_char);
        if ap.is_null() {
            ap = agattr(root, 2 as libc::c_int, name, defval);
        }
    }
    if is_html {
        let mut val: *mut libc::c_char = agstrdup_html(root, value);
        agxset(ep as *mut libc::c_void, ap, val);
        agstrfree(root, val);
    } else {
        agxset(ep as *mut libc::c_void, ap, value);
    };
}
unsafe extern "C" fn setGlobalEdgeAttr(
    mut g: *mut Agraph_t,
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
) {
    if strncmp(
        name,
        b"edge:\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) != 0
    {
        fprintf(
            stderr,
            b"Warning: global edge attribute %s in graph %s does not begin with the prefix %s\n\0"
                as *const u8 as *const libc::c_char,
            name,
            agnameof(g as *mut libc::c_void),
            b"edge:\0" as *const u8 as *const libc::c_char,
        );
    } else {
        name = name.offset(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
    }
    if g != root && (agattr(root, 2 as libc::c_int, name, 0 as *const libc::c_char)).is_null() {
        agattr(root, 2 as libc::c_int, name, defval);
    }
    agattr(g, 2 as libc::c_int, name, value);
}
unsafe extern "C" fn setGraphAttr(
    mut g: *mut Agraph_t,
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut ud: *mut userdata_t,
) {
    let mut ap: *mut Agsym_t = 0 as *mut Agsym_t;
    if g == root
        && strcmp(name, b"strict\0" as *const u8 as *const libc::c_char) == 0
        && strcmp(value, b"true\0" as *const u8 as *const libc::c_char) == 0
    {
        let ref mut fresh22 = (*g).desc;
        (*fresh22).set_strict(1 as libc::c_int as libc::c_uint);
    } else if strcmp(name, b"name\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        setName((*ud).nameMap, g as *mut Agobj_t, value);
    } else {
        ap = agattr(root, 0 as libc::c_int, name, 0 as *const libc::c_char);
        if !ap.is_null() {
            agxset(g as *mut libc::c_void, ap, value);
        } else if g == root {
            agattr(root, 0 as libc::c_int, name, value);
        } else {
            ap = agattr(root, 0 as libc::c_int, name, defval);
            agxset(g as *mut libc::c_void, ap, value);
        }
    };
}
unsafe extern "C" fn setAttr(
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut ud: *mut userdata_t,
    mut is_html: bool,
) {
    match Current_class as libc::c_uint {
        1 => {
            setGraphAttr(G, name, value, ud);
        }
        2 => {
            setNodeAttr(N, name, value, ud, is_html);
        }
        3 => {
            setEdgeAttr(E, name, value, ud, is_html);
        }
        _ => {}
    };
}
unsafe extern "C" fn startElementHandler(
    mut userData: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut atts: *mut *const libc::c_char,
) {
    let mut pos: libc::c_int = 0;
    let mut ud: *mut userdata_t = userData as *mut userdata_t;
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    if !(strcmp(name, b"gxl\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int) {
        if strcmp(name, b"graph\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            let mut edgeMode: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
            let mut buf: [libc::c_char; 100] = [0; 100];
            Current_class = TAG_GRAPH;
            if (*ud).closedElementType as libc::c_uint == TAG_GRAPH as libc::c_int as libc::c_uint {
                fprintf(
                    stderr,
                    b"Warning: Node contains more than one graph.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            pos = get_xml_attr(
                b"id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                atts,
            );
            if pos <= 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error: Graph has no ID attribute.\n\0" as *const u8 as *const libc::c_char,
                );
                graphviz_exit(1 as libc::c_int);
            }
            let mut id: *const libc::c_char = *atts.offset(pos as isize);
            pos = get_xml_attr(
                b"edgemode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                atts,
            );
            if pos > 0 as libc::c_int {
                edgeMode = *atts.offset(pos as isize);
            }
            if stack_is_empty(&mut Gstack) {
                if strcmp(edgeMode, b"directed\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    g = agopen(id as *mut libc::c_char, Agdirected, &mut AgDefaultDisc);
                } else if strcmp(
                    edgeMode,
                    b"undirected\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    g = agopen(id as *mut libc::c_char, Agundirected, &mut AgDefaultDisc);
                } else {
                    fprintf(
                        stderr,
                        b"Warning: graph has no edgemode attribute\0" as *const u8
                            as *const libc::c_char,
                    );
                    fprintf(
                        stderr,
                        b" - assume directed\n\0" as *const u8 as *const libc::c_char,
                    );
                    g = agopen(id as *mut libc::c_char, Agdirected, &mut AgDefaultDisc);
                }
                push_subg(g);
            } else {
                let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
                if isAnonGraph(id) != 0 {
                    static mut anon_id: libc::c_int = 1 as libc::c_int;
                    let fresh23 = anon_id;
                    anon_id = anon_id + 1;
                    snprintf(
                        buf.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
                        b"%%%d\0" as *const u8 as *const libc::c_char,
                        fresh23,
                    );
                    id = buf.as_mut_ptr();
                }
                subg = agsubg(G, id as *mut libc::c_char, 1 as libc::c_int);
                push_subg(subg);
            }
            pos = get_xml_attr(
                b"role\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                atts,
            );
            if pos > 0 as libc::c_int {
                setGraphAttr(
                    G,
                    b"_gxl_role\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    *atts.offset(pos as isize) as *mut libc::c_char,
                    ud,
                );
            }
            pos = get_xml_attr(
                b"hypergraph\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                atts,
            );
            if pos > 0 as libc::c_int {
                setGraphAttr(
                    G,
                    b"_gxl_hypergraph\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    *atts.offset(pos as isize) as *mut libc::c_char,
                    ud,
                );
            }
            pushString(&mut (*ud).elements, id);
        } else if strcmp(name, b"node\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            Current_class = TAG_NODE;
            pos = get_xml_attr(
                b"id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                atts,
            );
            if pos > 0 as libc::c_int {
                let mut attrname: *const libc::c_char = 0 as *const libc::c_char;
                attrname = *atts.offset(pos as isize);
                if !attrname.is_null()
                    && strcmp(attrname, b"\0" as *const u8 as *const libc::c_char)
                        != 0 as libc::c_int
                {
                    bind_node(attrname);
                    pushString(&mut (*ud).elements, attrname);
                }
            }
        } else if strcmp(name, b"edge\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            let mut head: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
            let mut tail: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
            let mut tname: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut t: *mut Agnode_t = 0 as *mut Agnode_t;
            Current_class = TAG_EDGE;
            pos = get_xml_attr(
                b"from\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                atts,
            );
            if pos > 0 as libc::c_int {
                tail = *atts.offset(pos as isize);
            }
            pos = get_xml_attr(
                b"to\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                atts,
            );
            if pos > 0 as libc::c_int {
                head = *atts.offset(pos as isize);
            }
            tname = mapLookup((*ud).nameMap, tail);
            if !tname.is_null() {
                tail = tname;
            }
            tname = mapLookup((*ud).nameMap, head);
            if !tname.is_null() {
                head = tname;
            }
            bind_edge(tail, head);
            t = (*if ((*(E as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                E
            } else {
                E.offset(1 as libc::c_int as isize)
            })
            .node;
            tname = agnameof(t as *mut libc::c_void);
            if strcmp(tname, tail) == 0 as libc::c_int {
                (*ud).edgeinverted = 0 as libc::c_int;
            } else if strcmp(tname, head) == 0 as libc::c_int {
                (*ud).edgeinverted = (0 as libc::c_int == 0) as libc::c_int;
            }
            pos = get_xml_attr(
                b"fromorder\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                atts,
            );
            if pos > 0 as libc::c_int {
                setEdgeAttr(
                    E,
                    b"_gxl_fromorder\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    *atts.offset(pos as isize) as *mut libc::c_char,
                    ud,
                    0 as libc::c_int != 0,
                );
            }
            pos = get_xml_attr(
                b"toorder\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                atts,
            );
            if pos > 0 as libc::c_int {
                setEdgeAttr(
                    E,
                    b"_gxl_toorder\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    *atts.offset(pos as isize) as *mut libc::c_char,
                    ud,
                    0 as libc::c_int != 0,
                );
            }
            pos = get_xml_attr(
                b"id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                atts,
            );
            if pos > 0 as libc::c_int {
                setEdgeAttr(
                    E,
                    b"_gxl_id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    *atts.offset(pos as isize) as *mut libc::c_char,
                    ud,
                    0 as libc::c_int != 0,
                );
            }
        } else if strcmp(name, b"attr\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            let mut attrname_0: *const libc::c_char = *atts.offset(get_xml_attr(
                b"name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                atts,
            ) as isize);
            agxbput(&mut (*ud).xml_attr_name, attrname_0);
            pos = get_xml_attr(
                b"kind\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                atts,
            );
            if pos > 0 as libc::c_int {
                if strcmp(
                    b"node\0" as *const u8 as *const libc::c_char,
                    *atts.offset(pos as isize),
                ) == 0 as libc::c_int
                {
                    (*ud).globalAttrType = TAG_NODE;
                } else if strcmp(
                    b"edge\0" as *const u8 as *const libc::c_char,
                    *atts.offset(pos as isize),
                ) == 0 as libc::c_int
                {
                    (*ud).globalAttrType = TAG_EDGE;
                } else if strcmp(
                    b"graph\0" as *const u8 as *const libc::c_char,
                    *atts.offset(pos as isize),
                ) == 0 as libc::c_int
                {
                    (*ud).globalAttrType = TAG_GRAPH;
                } else if strcmp(
                    b"HTML-like string\0" as *const u8 as *const libc::c_char,
                    *atts.offset(pos as isize),
                ) == 0 as libc::c_int
                {
                    (*ud).globalAttrType = TAG_HTML_LIKE_STRING;
                }
            } else {
                (*ud).globalAttrType = TAG_NONE;
            }
        } else if strcmp(name, b"string\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(name, b"bool\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(name, b"int\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(name, b"float\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*ud).listen = (0 as libc::c_int == 0) as libc::c_int;
            if (*ud).compositeReadState != 0 {
                agxbprint(
                    &mut (*ud).composite_buffer as *mut agxbuf,
                    b"<%s>\0" as *const u8 as *const libc::c_char,
                    name,
                );
            }
        } else if strcmp(name, b"rel\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(name, b"relend\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"%s element is ignored by DOT\n\0" as *const u8 as *const libc::c_char,
                name,
            );
        } else if strcmp(name, b"type\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            pos = get_xml_attr(
                b"xlink:href\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                atts,
            );
            if pos > 0 as libc::c_int {
                setAttr(
                    b"_gxl_type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    *atts.offset(pos as isize) as *mut libc::c_char,
                    ud,
                    0 as libc::c_int != 0,
                );
            }
        } else if strcmp(name, b"locator\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            pos = get_xml_attr(
                b"xlink:href\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                atts,
            );
            if pos > 0 as libc::c_int {
                let mut href: *const libc::c_char = *atts.offset(pos as isize);
                agxbprint(
                    &mut (*ud).xml_attr_value as *mut agxbuf,
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    b"_gxl_locator_\0" as *const u8 as *const libc::c_char,
                    href,
                );
            }
        } else if strcmp(name, b"seq\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(name, b"set\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(name, b"bag\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(name, b"tup\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(name, b"enum\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*ud).compositeReadState = (0 as libc::c_int == 0) as libc::c_int;
            agxbprint(
                &mut (*ud).composite_buffer as *mut agxbuf,
                b"<%s>\0" as *const u8 as *const libc::c_char,
                name,
            );
        } else {
            fprintf(
                stderr,
                b"Unknown node %s; DOT does not support extensions.\n\0" as *const u8
                    as *const libc::c_char,
                name,
            );
        }
    }
}
unsafe extern "C" fn endElementHandler(
    mut userData: *mut libc::c_void,
    mut name: *const libc::c_char,
) {
    let mut ud: *mut userdata_t = userData as *mut userdata_t;
    if strcmp(name, b"graph\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        pop_subg();
        popString(&mut (*ud).elements);
        (*ud).closedElementType = TAG_GRAPH;
    } else if strcmp(name, b"node\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        let mut ele_name: *mut libc::c_char = topString(&mut (*ud).elements);
        if (*ud).closedElementType as libc::c_uint == TAG_GRAPH as libc::c_int as libc::c_uint {
            let mut node: *mut Agnode_t = agnode(root, ele_name, 0 as libc::c_int);
            agdelete(root, node as *mut libc::c_void);
        }
        popString(&mut (*ud).elements);
        Current_class = TAG_GRAPH;
        N = 0 as *mut Agnode_t;
        (*ud).closedElementType = TAG_NODE;
    } else if strcmp(name, b"edge\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        Current_class = TAG_GRAPH;
        E = 0 as *mut Agedge_t;
        (*ud).closedElementType = TAG_EDGE;
        (*ud).edgeinverted = 0 as libc::c_int;
    } else if strcmp(name, b"attr\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        let mut new_name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut buf: [libc::c_char; 1000] = *::std::mem::transmute::<
            &[u8; 1000],
            &mut [libc::c_char; 1000],
        >(
            b"_gxl_composite_\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        let mut dynbuf: *mut libc::c_char = 0 as *mut libc::c_char;
        (*ud).closedElementType = TAG_NONE;
        if (*ud).compositeReadState != 0 {
            let mut len: size_t = (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                .wrapping_add(agxblen(&mut (*ud).xml_attr_name));
            if len <= 1000 as libc::c_int as libc::c_ulong {
                new_name = buf.as_mut_ptr();
            } else {
                dynbuf = gcalloc(len, ::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as *mut libc::c_char;
                new_name = dynbuf;
                strcpy(
                    new_name,
                    b"_gxl_composite_\0" as *const u8 as *const libc::c_char,
                );
            }
            strcpy(
                new_name
                    .offset(::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as isize)
                    .offset(-(1 as libc::c_int as isize)),
                agxbuse(&mut (*ud).xml_attr_name),
            );
            value = agxbuse(&mut (*ud).composite_buffer);
            agxbclear(&mut (*ud).xml_attr_value);
            (*ud).compositeReadState = 0 as libc::c_int;
        } else {
            new_name = agxbuse(&mut (*ud).xml_attr_name);
            value = agxbuse(&mut (*ud).xml_attr_value);
        }
        match (*ud).globalAttrType as libc::c_uint {
            0 => {
                setAttr(new_name, value, ud, 0 as libc::c_int != 0);
            }
            2 => {
                setGlobalNodeAttr(G, new_name, value);
            }
            3 => {
                setGlobalEdgeAttr(G, new_name, value);
            }
            1 => {
                setGraphAttr(G, new_name, value, ud);
            }
            4 => {
                setAttr(new_name, value, ud, 1 as libc::c_int != 0);
            }
            _ => {}
        }
        free(dynbuf as *mut libc::c_void);
        (*ud).globalAttrType = TAG_NONE;
    } else if strcmp(name, b"string\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(name, b"bool\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(name, b"int\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(name, b"float\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        (*ud).listen = 0 as libc::c_int;
        if (*ud).compositeReadState != 0 {
            agxbprint(
                &mut (*ud).composite_buffer as *mut agxbuf,
                b"</%s>\0" as *const u8 as *const libc::c_char,
                name,
            );
        }
    } else if strcmp(name, b"seq\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(name, b"set\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(name, b"bag\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(name, b"tup\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(name, b"enum\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        agxbprint(
            &mut (*ud).composite_buffer as *mut agxbuf,
            b"</%s>\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
}
unsafe extern "C" fn characterDataHandler(
    mut userData: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut length: libc::c_int,
) {
    let mut ud: *mut userdata_t = userData as *mut userdata_t;
    if length >= 0 as libc::c_int
        && !(b"Expat returned negative length data\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"length >= 0 && \"Expat returned negative length data\"\0" as *const u8
                as *const libc::c_char,
            b"gxl2gv.c\0" as *const u8 as *const libc::c_char,
            689 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                b"void characterDataHandler(void *, const char *, int)\0",
            ))
            .as_ptr(),
        );
    }
    let mut len: size_t = length as size_t;
    if (*ud).listen == 0 {
        return;
    }
    if (*ud).compositeReadState != 0 {
        agxbput_n(&mut (*ud).composite_buffer, s, len);
        return;
    }
    agxbput_n(&mut (*ud).xml_attr_value, s, len);
}
#[no_mangle]
pub unsafe extern "C" fn gxl_to_gv(mut gxlFile: *mut FILE) -> *mut Agraph_t {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut done: libc::c_int = 0;
    let mut udata: *mut userdata_t = genUserdata();
    let mut parser: XML_Parser = XML_ParserCreate(0 as *const XML_Char);
    if udata.is_null() {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    XML_SetUserData(parser, udata as *mut libc::c_void);
    XML_SetElementHandler(
        parser,
        Some(
            startElementHandler
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    *mut *const libc::c_char,
                ) -> (),
        ),
        Some(
            endElementHandler as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> (),
        ),
    );
    XML_SetCharacterDataHandler(
        parser,
        Some(
            characterDataHandler
                as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, libc::c_int) -> (),
        ),
    );
    Current_class = TAG_GRAPH;
    root = 0 as *mut Agraph_t;
    loop {
        let mut len: size_t = fread(
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            gxlFile,
        );
        if len == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        done =
            (len < ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong) as libc::c_int;
        if len <= 2147483647 as libc::c_int as size_t
            && !(b"too large data for Expat API\0" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"len <= (size_t)INT_MAX && \"too large data for Expat API\"\0" as *const u8
                    as *const libc::c_char,
                b"gxl2gv.c\0" as *const u8 as *const libc::c_char,
                726 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    b"Agraph_t *gxl_to_gv(FILE *)\0",
                ))
                .as_ptr(),
            );
        }
        if XML_Parse(parser, buf.as_mut_ptr(), len as libc::c_int, done) as libc::c_uint
            == XML_STATUS_ERROR as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s at line %lu\n\0" as *const u8 as *const libc::c_char,
                XML_ErrorString(XML_GetErrorCode(parser)),
                XML_GetCurrentLineNumber(parser),
            );
            graphviz_exit(1 as libc::c_int);
        }
        if !(done == 0) {
            break;
        }
    }
    XML_ParserFree(parser);
    freeUserdata(udata);
    stack_reset(&mut Gstack);
    return root;
}
