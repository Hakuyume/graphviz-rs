#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _GHashTable;
    pub type _GtsConstraintClass;
    pub type _GtsConstraint;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn gts_constraint_class() -> *mut GtsConstraintClass;
    fn g_return_if_fail_warning(
        log_domain: *const libc::c_char,
        pretty_function: *const libc::c_char,
        expression: *const libc::c_char,
    );
    fn gts_triangle_vertices_edges(
        t: *mut GtsTriangle,
        e: *mut GtsEdge,
        v1: *mut *mut GtsVertex,
        v2: *mut *mut GtsVertex,
        v3: *mut *mut GtsVertex,
        e1: *mut *mut GtsEdge,
        e2: *mut *mut GtsEdge,
        e3: *mut *mut GtsEdge,
    );
    fn gts_surface_foreach_face_remove(
        s: *mut GtsSurface,
        func: GtsFunc,
        data: gpointer,
    ) -> guint;
    static mut gts_allow_floating_vertices: gboolean;
    static mut gts_allow_floating_edges: gboolean;
    fn gts_object_destroy(object: *mut GtsObject);
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
    fn gts_delaunay_add_constraint(
        surface: *mut GtsSurface,
        c: *mut GtsConstraint,
    ) -> *mut GSList;
    fn gts_delaunay_add_vertex(
        surface: *mut GtsSurface,
        v: *mut GtsVertex,
        guess: *mut GtsFace,
    ) -> *mut GtsVertex;
    fn gts_vertex_replace(v: *mut GtsVertex, with: *mut GtsVertex);
    fn gts_face_class() -> *mut GtsFaceClass;
    fn gts_face_new(
        klass: *mut GtsFaceClass,
        e1: *mut GtsEdge,
        e2: *mut GtsEdge,
        e3: *mut GtsEdge,
    ) -> *mut GtsFace;
    fn gts_surface_add_face(s: *mut GtsSurface, f: *mut GtsFace);
    fn gts_vertex_class() -> *mut GtsVertexClass;
    fn gts_edge_class() -> *mut GtsEdgeClass;
    fn gts_object_class_new(
        parent_class: *mut GtsObjectClass,
        info: *mut GtsObjectClassInfo,
    ) -> gpointer;
    fn gts_surface_class() -> *mut GtsSurfaceClass;
    fn gts_surface_new(
        klass: *mut GtsSurfaceClass,
        face_class: *mut GtsFaceClass,
        edge_class: *mut GtsEdgeClass,
        vertex_class: *mut GtsVertexClass,
    ) -> *mut GtsSurface;
    fn gts_triangle_vertices(
        t: *mut GtsTriangle,
        v1: *mut *mut GtsVertex,
        v2: *mut *mut GtsVertex,
        v3: *mut *mut GtsVertex,
    );
    fn g_slist_free(list: *mut GSList);
    fn gts_triangle_class() -> *mut GtsTriangleClass;
    fn gts_triangle_enclosing(
        klass: *mut GtsTriangleClass,
        points: *mut GSList,
        scale: gdouble,
    ) -> *mut GtsTriangle;
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn gts_edge_new(
        klass: *mut GtsEdgeClass,
        v1: *mut GtsVertex,
        v2: *mut GtsVertex,
    ) -> *mut GtsEdge;
    fn gts_vertex_new(
        klass: *mut GtsVertexClass,
        x: gdouble,
        y: gdouble,
        z: gdouble,
    ) -> *mut GtsVertex;
    fn gts_surface_foreach_edge(s: *mut GtsSurface, func: GtsFunc, data: gpointer);
    fn gts_surface_foreach_face(s: *mut GtsSurface, func: GtsFunc, data: gpointer);
    fn gts_face_foreach_neighbor(
        f: *mut GtsFace,
        s: *mut GtsSurface,
        func: GtsFunc,
        data: gpointer,
    );
}
pub type size_t = libc::c_ulong;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v_data {
    pub nedges: libc::c_int,
    pub edges: *mut libc::c_int,
    pub ewgts: *mut libc::c_float,
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
pub struct surface_t {
    pub nedges: libc::c_int,
    pub edges: *mut libc::c_int,
    pub nfaces: libc::c_int,
    pub faces: *mut libc::c_int,
    pub neigh: *mut libc::c_int,
}
pub type GtsObject = _GtsObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsObject {
    pub klass: *mut GtsObjectClass,
    pub reserved: gpointer,
    pub flags: guint32,
}
pub type guint32 = libc::c_uint;
pub type gpointer = *mut libc::c_void;
pub type GtsObjectClass = _GtsObjectClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsObjectClass {
    pub info: GtsObjectClassInfo,
    pub parent_class: *mut GtsObjectClass,
    pub clone: Option::<unsafe extern "C" fn(*mut GtsObject, *mut GtsObject) -> ()>,
    pub destroy: Option::<unsafe extern "C" fn(*mut GtsObject) -> ()>,
    pub read: Option::<unsafe extern "C" fn(*mut *mut GtsObject, *mut GtsFile) -> ()>,
    pub write: Option::<unsafe extern "C" fn(*mut GtsObject, *mut FILE) -> ()>,
    pub color: Option::<unsafe extern "C" fn(*mut GtsObject) -> GtsColor>,
    pub attributes: Option::<unsafe extern "C" fn(*mut GtsObject, *mut GtsObject) -> ()>,
}
pub type GtsColor = _GtsColor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsColor {
    pub r: gfloat,
    pub g: gfloat,
    pub b: gfloat,
}
pub type gfloat = libc::c_float;
pub type GtsFile = _GtsFile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsFile {
    pub fp: *mut FILE,
    pub line: guint,
    pub pos: guint,
    pub token: *mut GString,
    pub type_0: GtsTokenType,
    pub error: *mut gchar,
    pub curline: guint,
    pub curpos: guint,
    pub scope: guint,
    pub scope_max: guint,
    pub next_token: gint,
    pub delimiters: *mut gchar,
    pub comments: *mut gchar,
    pub tokens: *mut gchar,
    pub buf: *mut gchar,
    pub len: size_t,
}
pub type gchar = libc::c_char;
pub type gint = libc::c_int;
pub type guint = libc::c_uint;
pub type GtsTokenType = libc::c_uint;
pub const GTS_OBJ: GtsTokenType = 65536;
pub const GTS_ERROR: GtsTokenType = 32768;
pub const GTS_FILE: GtsTokenType = 16384;
pub const GTS_STRING: GtsTokenType = 8192;
pub const GTS_DOUBLE: GtsTokenType = 4096;
pub const GTS_FLOAT: GtsTokenType = 2048;
pub const GTS_UINT: GtsTokenType = 1024;
pub const GTS_INT: GtsTokenType = 512;
pub const GTS_NONE: GtsTokenType = 256;
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type gsize = libc::c_ulong;
pub type GtsObjectClassInfo = _GtsObjectClassInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsObjectClassInfo {
    pub name: [gchar; 40],
    pub object_size: guint,
    pub class_size: guint,
    pub class_init_func: GtsObjectClassInitFunc,
    pub object_init_func: GtsObjectInitFunc,
    pub arg_set_func: GtsArgSetFunc,
    pub arg_get_func: GtsArgGetFunc,
}
pub type GtsArgGetFunc = Option::<unsafe extern "C" fn(*mut GtsObject) -> ()>;
pub type GtsArgSetFunc = Option::<unsafe extern "C" fn(*mut GtsObject) -> ()>;
pub type GtsObjectInitFunc = Option::<unsafe extern "C" fn(*mut GtsObject) -> ()>;
pub type GtsObjectClassInitFunc = Option::<
    unsafe extern "C" fn(*mut GtsObjectClass) -> (),
>;
pub type GtsSurface = _GtsSurface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsSurface {
    pub object: GtsObject,
    pub faces: *mut GHashTable,
    pub face_class: *mut GtsFaceClass,
    pub edge_class: *mut GtsEdgeClass,
    pub vertex_class: *mut GtsVertexClass,
    pub keep_faces: gboolean,
}
pub type gboolean = gint;
pub type GtsVertexClass = _GtsVertexClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsVertexClass {
    pub parent_class: GtsPointClass,
    pub intersection_attributes: Option::<
        unsafe extern "C" fn(*mut GtsVertex, *mut GtsObject, *mut GtsObject) -> (),
    >,
}
pub type GtsVertex = _GtsVertex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsVertex {
    pub p: GtsPoint,
    pub segments: *mut GSList,
}
pub type GSList = _GSList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GtsPoint = _GtsPoint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsPoint {
    pub object: GtsObject,
    pub x: gdouble,
    pub y: gdouble,
    pub z: gdouble,
}
pub type gdouble = libc::c_double;
pub type GtsPointClass = _GtsPointClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsPointClass {
    pub parent_class: GtsObjectClass,
    pub binary: gboolean,
}
pub type GtsEdgeClass = _GtsEdgeClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsEdgeClass {
    pub parent_class: GtsSegmentClass,
}
pub type GtsSegmentClass = _GtsSegmentClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsSegmentClass {
    pub parent_class: GtsObjectClass,
}
pub type GtsFaceClass = _GtsFaceClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsFaceClass {
    pub parent_class: GtsTriangleClass,
}
pub type GtsTriangleClass = _GtsTriangleClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsTriangleClass {
    pub parent_class: GtsObjectClass,
}
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GVertex {
    pub v: GtsVertex,
    pub idx: libc::c_int,
}
pub type GtsEdge = _GtsEdge;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsEdge {
    pub segment: GtsSegment,
    pub triangles: *mut GSList,
}
pub type GtsSegment = _GtsSegment;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsSegment {
    pub object: GtsObject,
    pub v1: *mut GtsVertex,
    pub v2: *mut GtsVertex,
}
pub type GtsConstraintClass = _GtsConstraintClass;
pub type GtsTriangle = _GtsTriangle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsTriangle {
    pub object: GtsObject,
    pub e1: *mut GtsEdge,
    pub e2: *mut GtsEdge,
    pub e3: *mut GtsEdge,
}
pub type GtsFunc = Option::<unsafe extern "C" fn(gpointer, gpointer) -> gint>;
pub type GtsConstraint = _GtsConstraint;
pub type GtsFace = _GtsFace;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsFace {
    pub triangle: GtsTriangle,
    pub surfaces: *mut GSList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GFaceClass {
    pub parent_class: GtsFaceClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GFace {
    pub v: GtsFace,
    pub idx: libc::c_int,
}
pub type GtsSurfaceClass = _GtsSurfaceClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtsSurfaceClass {
    pub parent_class: GtsObjectClass,
    pub add_face: Option::<unsafe extern "C" fn(*mut GtsSurface, *mut GtsFace) -> ()>,
    pub remove_face: Option::<unsafe extern "C" fn(*mut GtsSurface, *mut GtsFace) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GVertexClass {
    pub parent_class: GtsVertexClass,
}
pub type qsort_cmpf = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct estate {
    pub n: libc::c_int,
    pub edges: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct estats {
    pub n: libc::c_int,
    pub delaunay: *mut v_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fstate {
    pub s: *mut GtsSurface,
    pub faces: *mut libc::c_int,
    pub neigh: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ninfo {
    pub nneigh: libc::c_int,
    pub neigh: *mut libc::c_int,
}
#[inline]
unsafe extern "C" fn gts_object_is_from_class(
    mut object: gpointer,
    mut klass: gpointer,
) -> gpointer {
    let mut c: *mut GtsObjectClass = 0 as *mut GtsObjectClass;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !klass.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            0 as *mut gchar,
            (*::std::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"gts_object_is_from_class\0"))
                .as_ptr(),
            b"klass != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_void;
    }
    if object.is_null() {
        return 0 as *mut libc::c_void;
    }
    c = (*(object as *mut GtsObject)).klass;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !c.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            0 as *mut gchar,
            (*::std::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"gts_object_is_from_class\0"))
                .as_ptr(),
            b"c != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_void;
    }
    while !c.is_null() {
        if c == klass as *mut GtsObjectClass {
            return object;
        }
        c = (*c).parent_class;
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn triangle_is_hole(
    mut triangle: *mut libc::c_void,
    mut ignored: *mut libc::c_void,
) -> gint {
    let mut t: *mut GtsTriangle = triangle as *mut GtsTriangle;
    let mut e1: *mut GtsEdge = 0 as *mut GtsEdge;
    let mut e2: *mut GtsEdge = 0 as *mut GtsEdge;
    let mut e3: *mut GtsEdge = 0 as *mut GtsEdge;
    let mut v1: *mut GtsVertex = 0 as *mut GtsVertex;
    let mut v2: *mut GtsVertex = 0 as *mut GtsVertex;
    let mut v3: *mut GtsVertex = 0 as *mut GtsVertex;
    gts_triangle_vertices_edges(
        t,
        0 as *mut GtsEdge,
        &mut v1,
        &mut v2,
        &mut v3,
        &mut e1,
        &mut e2,
        &mut e3,
    );
    if !(gts_object_is_from_class(e1 as gpointer, gts_constraint_class() as gpointer))
        .is_null() && (*(e1 as *mut GtsSegment)).v1 != v1
        || !(gts_object_is_from_class(
            e2 as gpointer,
            gts_constraint_class() as gpointer,
        ))
            .is_null() && (*(e2 as *mut GtsSegment)).v1 != v2
        || !(gts_object_is_from_class(
            e3 as gpointer,
            gts_constraint_class() as gpointer,
        ))
            .is_null() && (*(e3 as *mut GtsSegment)).v1 != v3
    {
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn delaunay_remove_holes(mut surface: *mut GtsSurface) -> guint {
    return gts_surface_foreach_face_remove(
        surface,
        Some(
            triangle_is_hole
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> gint,
        ),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn g_vertex_class() -> *mut GVertexClass {
    static mut klass: *mut GVertexClass = 0 as *const GVertexClass as *mut GVertexClass;
    if klass.is_null() {
        let mut vertex_info: GtsObjectClassInfo = {
            let mut init = _GtsObjectClassInfo {
                name: *::std::mem::transmute::<
                    &[u8; 40],
                    &mut [gchar; 40],
                >(
                    b"GVertex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
                object_size: ::std::mem::size_of::<GVertex>() as libc::c_ulong as guint,
                class_size: ::std::mem::size_of::<GVertexClass>() as libc::c_ulong
                    as guint,
                class_init_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    GtsObjectClassInitFunc,
                >(0 as *mut libc::c_void),
                object_init_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    GtsObjectInitFunc,
                >(0 as *mut libc::c_void),
                arg_set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    GtsArgSetFunc,
                >(0 as *mut libc::c_void),
                arg_get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    GtsArgGetFunc,
                >(0 as *mut libc::c_void),
            };
            init
        };
        klass = gts_object_class_new(
            gts_vertex_class() as *mut GtsObjectClass,
            &mut vertex_info,
        ) as *mut GVertexClass;
    }
    return klass;
}
unsafe extern "C" fn g_face_class() -> *mut GFaceClass {
    static mut klass: *mut GFaceClass = 0 as *const GFaceClass as *mut GFaceClass;
    if klass.is_null() {
        let mut face_info: GtsObjectClassInfo = {
            let mut init = _GtsObjectClassInfo {
                name: *::std::mem::transmute::<
                    &[u8; 40],
                    &mut [gchar; 40],
                >(
                    b"GFace\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
                object_size: ::std::mem::size_of::<GFace>() as libc::c_ulong as guint,
                class_size: ::std::mem::size_of::<GFaceClass>() as libc::c_ulong
                    as guint,
                class_init_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    GtsObjectClassInitFunc,
                >(0 as *mut libc::c_void),
                object_init_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    GtsObjectInitFunc,
                >(0 as *mut libc::c_void),
                arg_set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    GtsArgSetFunc,
                >(0 as *mut libc::c_void),
                arg_get_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    GtsArgGetFunc,
                >(0 as *mut libc::c_void),
            };
            init
        };
        klass = gts_object_class_new(
            gts_face_class() as *mut GtsObjectClass,
            &mut face_info,
        ) as *mut GFaceClass;
    }
    return klass;
}
unsafe extern "C" fn destroy(mut v: *mut GtsVertex) {
    let mut i: *mut GSList = 0 as *mut GSList;
    i = (*v).segments;
    while !i.is_null() {
        let mut next: *mut GSList = (*i).next;
        gts_object_destroy((*i).data as *mut GtsObject);
        i = next;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if ((*v).segments).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"delaunay.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"destroy\0"))
                .as_ptr(),
            b"v->segments == NULL\0" as *const u8 as *const libc::c_char,
        );
    }
    gts_object_destroy(v as *mut GtsObject);
}
unsafe extern "C" fn tri(
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut npt: libc::c_int,
    mut segs: *mut libc::c_int,
    mut nsegs: libc::c_int,
    mut sepArr: libc::c_int,
) -> *mut GtsSurface {
    let mut i: libc::c_int = 0;
    let mut surface: *mut GtsSurface = 0 as *mut GtsSurface;
    let mut vertices: *mut *mut GVertex = gcalloc(
        npt as size_t,
        ::std::mem::size_of::<*mut GVertex>() as libc::c_ulong,
    ) as *mut *mut GVertex;
    let mut edges: *mut *mut GtsEdge = gcalloc(
        nsegs as size_t,
        ::std::mem::size_of::<*mut GtsEdge>() as libc::c_ulong,
    ) as *mut *mut GtsEdge;
    let mut list: *mut GSList = 0 as *mut GSList;
    let mut v1: *mut GtsVertex = 0 as *mut GtsVertex;
    let mut v2: *mut GtsVertex = 0 as *mut GtsVertex;
    let mut v3: *mut GtsVertex = 0 as *mut GtsVertex;
    let mut t: *mut GtsTriangle = 0 as *mut GtsTriangle;
    let mut vcl: *mut GtsVertexClass = g_vertex_class() as *mut GtsVertexClass;
    let mut ecl: *mut GtsEdgeClass = gts_constraint_class() as *mut GtsEdgeClass;
    if sepArr != 0 {
        i = 0 as libc::c_int;
        while i < npt {
            let mut p: *mut GVertex = gts_vertex_new(
                vcl,
                *x.offset(i as isize),
                *y.offset(i as isize),
                0 as libc::c_int as gdouble,
            ) as *mut GVertex;
            (*p).idx = i;
            let ref mut fresh0 = *vertices.offset(i as isize);
            *fresh0 = p;
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < npt {
            let mut p_0: *mut GVertex = gts_vertex_new(
                vcl,
                *x.offset((2 as libc::c_int * i) as isize),
                *x.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize),
                0 as libc::c_int as gdouble,
            ) as *mut GVertex;
            (*p_0).idx = i;
            let ref mut fresh1 = *vertices.offset(i as isize);
            *fresh1 = p_0;
            i += 1;
        }
    }
    i = 0 as libc::c_int;
    while i < nsegs {
        let ref mut fresh2 = *edges.offset(i as isize);
        *fresh2 = gts_edge_new(
            ecl,
            *vertices.offset(*segs.offset((2 as libc::c_int * i) as isize) as isize)
                as *mut GtsVertex,
            *vertices
                .offset(
                    *segs.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize)
                        as isize,
                ) as *mut GtsVertex,
        );
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < npt {
        list = g_slist_prepend(list, *vertices.offset(i as isize) as gpointer);
        i += 1;
    }
    t = gts_triangle_enclosing(gts_triangle_class(), list, 100.0f64);
    g_slist_free(list);
    gts_triangle_vertices(t, &mut v1, &mut v2, &mut v3);
    surface = gts_surface_new(
        gts_surface_class(),
        g_face_class() as *mut GtsFaceClass,
        gts_edge_class(),
        gts_vertex_class(),
    );
    gts_surface_add_face(
        surface,
        gts_face_new(gts_face_class(), (*t).e1, (*t).e2, (*t).e3),
    );
    i = 0 as libc::c_int;
    while i < npt {
        let mut v4: *mut GtsVertex = *vertices.offset(i as isize) as *mut GtsVertex;
        let mut v: *mut GtsVertex = gts_delaunay_add_vertex(
            surface,
            v4,
            0 as *mut GtsFace,
        );
        if !v.is_null() {
            gts_vertex_replace(v4, v);
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < nsegs {
        gts_delaunay_add_constraint(
            surface,
            *edges.offset(i as isize) as *mut GtsConstraint,
        );
        i += 1;
    }
    gts_allow_floating_vertices = (0 as libc::c_int == 0) as libc::c_int;
    gts_allow_floating_edges = (0 as libc::c_int == 0) as libc::c_int;
    destroy(v1);
    destroy(v2);
    destroy(v3);
    gts_allow_floating_edges = 0 as libc::c_int;
    gts_allow_floating_vertices = 0 as libc::c_int;
    if nsegs != 0 {
        delaunay_remove_holes(surface);
    }
    free(edges as *mut libc::c_void);
    free(vertices as *mut libc::c_void);
    return surface;
}
unsafe extern "C" fn cnt_edge(
    mut edge: *mut libc::c_void,
    mut stats: *mut libc::c_void,
) -> gint {
    let mut e: *mut GtsSegment = edge as *mut GtsSegment;
    let mut sp: *mut estats = stats as *mut estats;
    let ref mut fresh3 = (*sp).n;
    *fresh3 += 1;
    if !((*sp).delaunay).is_null() {
        let ref mut fresh4 = (*((*sp).delaunay)
            .offset((*((*e).v1 as *mut GVertex)).idx as isize))
            .nedges;
        *fresh4 += 1;
        let ref mut fresh5 = (*((*sp).delaunay)
            .offset((*((*e).v2 as *mut GVertex)).idx as isize))
            .nedges;
        *fresh5 += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn edgeStats(mut s: *mut GtsSurface, mut sp: *mut estats) {
    gts_surface_foreach_edge(
        s,
        Some(
            cnt_edge
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> gint,
        ),
        sp as gpointer,
    );
}
unsafe extern "C" fn add_edge(
    mut edge: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> gint {
    let mut e: *mut GtsSegment = edge as *mut GtsSegment;
    let mut delaunay: *mut v_data = data as *mut v_data;
    let mut source: libc::c_int = (*((*e).v1 as *mut GVertex)).idx;
    let mut dest: libc::c_int = (*((*e).v2 as *mut GVertex)).idx;
    let ref mut fresh6 = (*delaunay.offset(source as isize)).nedges;
    let fresh7 = *fresh6;
    *fresh6 = *fresh6 + 1;
    *((*delaunay.offset(source as isize)).edges).offset(fresh7 as isize) = dest;
    let ref mut fresh8 = (*delaunay.offset(dest as isize)).nedges;
    let fresh9 = *fresh8;
    *fresh8 = *fresh8 + 1;
    *((*delaunay.offset(dest as isize)).edges).offset(fresh9 as isize) = source;
    return 0 as libc::c_int;
}
unsafe extern "C" fn delaunay_triangulation(
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut n: libc::c_int,
) -> *mut v_data {
    let mut delaunay: *mut v_data = 0 as *mut v_data;
    let mut s: *mut GtsSurface = tri(
        x,
        y,
        n,
        0 as *mut libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    let mut i: libc::c_int = 0;
    let mut nedges: libc::c_int = 0;
    let mut edges: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut stats: estats = estats {
        n: 0,
        delaunay: 0 as *mut v_data,
    };
    if s.is_null() {
        return 0 as *mut v_data;
    }
    delaunay = gcalloc(n as size_t, ::std::mem::size_of::<v_data>() as libc::c_ulong)
        as *mut v_data;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh10 = (*delaunay.offset(i as isize)).ewgts;
        *fresh10 = 0 as *mut libc::c_float;
        (*delaunay.offset(i as isize)).nedges = 1 as libc::c_int;
        i += 1;
    }
    stats.n = 0 as libc::c_int;
    stats.delaunay = delaunay;
    edgeStats(s, &mut stats);
    nedges = stats.n;
    edges = gcalloc(
        (2 as libc::c_int * nedges + n) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh11 = (*delaunay.offset(i as isize)).edges;
        *fresh11 = edges;
        edges = edges.offset((*delaunay.offset(i as isize)).nedges as isize);
        *((*delaunay.offset(i as isize)).edges).offset(0 as libc::c_int as isize) = i;
        (*delaunay.offset(i as isize)).nedges = 1 as libc::c_int;
        i += 1;
    }
    gts_surface_foreach_edge(
        s,
        Some(
            add_edge
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> gint,
        ),
        delaunay as gpointer,
    );
    gts_object_destroy(s as *mut GtsObject);
    return delaunay;
}
unsafe extern "C" fn addEdge(
    mut edge: *mut libc::c_void,
    mut state: *mut libc::c_void,
) -> gint {
    let mut e: *mut GtsSegment = edge as *mut GtsSegment;
    let mut es: *mut estate = state as *mut estate;
    let mut source: libc::c_int = (*((*e).v1 as *mut GVertex)).idx;
    let mut dest: libc::c_int = (*((*e).v2 as *mut GVertex)).idx;
    *((*es).edges).offset((2 as libc::c_int * (*es).n) as isize) = source;
    *((*es).edges)
        .offset((2 as libc::c_int * (*es).n + 1 as libc::c_int) as isize) = dest;
    (*es).n += 1 as libc::c_int;
    return 0 as libc::c_int;
}
static mut _vals: *mut libc::c_double = 0 as *const libc::c_double
    as *mut libc::c_double;
unsafe extern "C" fn vcmp(
    mut a: *mut libc::c_int,
    mut b: *mut libc::c_int,
) -> libc::c_int {
    let mut va: libc::c_double = *_vals.offset(*a as isize);
    let mut vb: libc::c_double = *_vals.offset(*b as isize);
    if va < vb {
        return -(1 as libc::c_int)
    } else if va > vb {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn delaunay_tri(
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut n: libc::c_int,
    mut pnedges: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut s: *mut GtsSurface = tri(
        x,
        y,
        n,
        0 as *mut libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    let mut nedges: libc::c_int = 0;
    let mut edges: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut stats: estats = estats {
        n: 0,
        delaunay: 0 as *mut v_data,
    };
    let mut state: estate = estate {
        n: 0,
        edges: 0 as *mut libc::c_int,
    };
    if s.is_null() {
        return 0 as *mut libc::c_int;
    }
    stats.n = 0 as libc::c_int;
    stats.delaunay = 0 as *mut v_data;
    edgeStats(s, &mut stats);
    nedges = stats.n;
    *pnedges = nedges;
    if nedges != 0 {
        edges = gcalloc(
            (2 as libc::c_int * nedges) as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        state.n = 0 as libc::c_int;
        state.edges = edges;
        gts_surface_foreach_edge(
            s,
            Some(
                addEdge
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> gint,
            ),
            &mut state as *mut estate as gpointer,
        );
    } else {
        let mut vs: *mut libc::c_int = gcalloc(
            n as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        let mut ip: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut i: libc::c_int = 0;
        let mut hd: libc::c_int = 0;
        let mut tl: libc::c_int = 0;
        nedges = n - 1 as libc::c_int;
        *pnedges = nedges;
        edges = gcalloc(
            (2 as libc::c_int * nedges) as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        ip = edges;
        i = 0 as libc::c_int;
        while i < n {
            *vs.offset(i as isize) = i;
            i += 1;
        }
        if *x.offset(0 as libc::c_int as isize) == *x.offset(1 as libc::c_int as isize) {
            _vals = y;
        } else {
            _vals = x;
        }
        qsort(
            vs as *mut libc::c_void,
            n as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_int,
                        *mut libc::c_int,
                    ) -> libc::c_int,
                >,
                qsort_cmpf,
            >(
                Some(
                    vcmp
                        as unsafe extern "C" fn(
                            *mut libc::c_int,
                            *mut libc::c_int,
                        ) -> libc::c_int,
                ),
            ),
        );
        tl = *vs.offset(0 as libc::c_int as isize);
        i = 1 as libc::c_int;
        while i < n {
            hd = *vs.offset(i as isize);
            let fresh12 = ip;
            ip = ip.offset(1);
            *fresh12 = tl;
            let fresh13 = ip;
            ip = ip.offset(1);
            *fresh13 = hd;
            tl = hd;
            i += 1;
        }
        free(vs as *mut libc::c_void);
    }
    gts_object_destroy(s as *mut GtsObject);
    return edges;
}
unsafe extern "C" fn cntFace(
    mut face: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> gint {
    let mut fp: *mut GFace = face as *mut GFace;
    let mut ip: *mut libc::c_int = data as *mut libc::c_int;
    (*fp).idx = *ip;
    *ip += 1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn addNeighbor(
    mut face: *mut libc::c_void,
    mut ni: *mut libc::c_void,
) -> gint {
    let mut f: *mut GFace = face as *mut GFace;
    let mut es: *mut ninfo = ni as *mut ninfo;
    *((*es).neigh).offset((*es).nneigh as isize) = (*f).idx;
    let ref mut fresh14 = (*es).nneigh;
    *fresh14 += 1;
    return 0 as libc::c_int;
}
unsafe extern "C" fn addFace(
    mut face: *mut libc::c_void,
    mut state: *mut libc::c_void,
) -> gint {
    let mut f: *mut GFace = face as *mut GFace;
    let mut es: *mut fstate = state as *mut fstate;
    let mut i: libc::c_int = 0;
    let mut myid: libc::c_int = (*f).idx;
    let mut ip: *mut libc::c_int = ((*es).faces)
        .offset((3 as libc::c_int * myid) as isize);
    let mut neigh: *mut libc::c_int = ((*es).neigh)
        .offset((3 as libc::c_int * myid) as isize);
    let mut ni: ninfo = ninfo {
        nneigh: 0,
        neigh: 0 as *mut libc::c_int,
    };
    let mut v1: *mut GtsVertex = 0 as *mut GtsVertex;
    let mut v2: *mut GtsVertex = 0 as *mut GtsVertex;
    let mut v3: *mut GtsVertex = 0 as *mut GtsVertex;
    gts_triangle_vertices(&mut (*f).v.triangle, &mut v1, &mut v2, &mut v3);
    let fresh15 = ip;
    ip = ip.offset(1);
    *fresh15 = (*(v1 as *mut GVertex)).idx;
    let fresh16 = ip;
    ip = ip.offset(1);
    *fresh16 = (*(v2 as *mut GVertex)).idx;
    let fresh17 = ip;
    ip = ip.offset(1);
    *fresh17 = (*(v3 as *mut GVertex)).idx;
    ni.nneigh = 0 as libc::c_int;
    ni.neigh = neigh;
    gts_face_foreach_neighbor(
        f as *mut GtsFace,
        0 as *mut GtsSurface,
        Some(
            addNeighbor
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> gint,
        ),
        &mut ni as *mut ninfo as gpointer,
    );
    i = ni.nneigh;
    while i < 3 as libc::c_int {
        *neigh.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn addTri(
    mut face: *mut libc::c_void,
    mut state: *mut libc::c_void,
) -> gint {
    let mut f: *mut GFace = face as *mut GFace;
    let mut es: *mut fstate = state as *mut fstate;
    let mut myid: libc::c_int = (*f).idx;
    let mut ip: *mut libc::c_int = ((*es).faces)
        .offset((3 as libc::c_int * myid) as isize);
    let mut v1: *mut GtsVertex = 0 as *mut GtsVertex;
    let mut v2: *mut GtsVertex = 0 as *mut GtsVertex;
    let mut v3: *mut GtsVertex = 0 as *mut GtsVertex;
    gts_triangle_vertices(&mut (*f).v.triangle, &mut v1, &mut v2, &mut v3);
    let fresh18 = ip;
    ip = ip.offset(1);
    *fresh18 = (*(v1 as *mut GVertex)).idx;
    let fresh19 = ip;
    ip = ip.offset(1);
    *fresh19 = (*(v2 as *mut GVertex)).idx;
    let fresh20 = ip;
    ip = ip.offset(1);
    *fresh20 = (*(v3 as *mut GVertex)).idx;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mkSurface(
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut n: libc::c_int,
    mut segs: *mut libc::c_int,
    mut nsegs: libc::c_int,
) -> *mut surface_t {
    let mut s: *mut GtsSurface = tri(x, y, n, segs, nsegs, 1 as libc::c_int);
    let mut stats: estats = estats {
        n: 0,
        delaunay: 0 as *mut v_data,
    };
    let mut state: estate = estate {
        n: 0,
        edges: 0 as *mut libc::c_int,
    };
    let mut statf: fstate = fstate {
        s: 0 as *mut GtsSurface,
        faces: 0 as *mut libc::c_int,
        neigh: 0 as *mut libc::c_int,
    };
    let mut sf: *mut surface_t = 0 as *mut surface_t;
    let mut nfaces: libc::c_int = 0 as libc::c_int;
    let mut faces: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut neigh: *mut libc::c_int = 0 as *mut libc::c_int;
    if s.is_null() {
        return 0 as *mut surface_t;
    }
    sf = gmalloc(::std::mem::size_of::<surface_t>() as libc::c_ulong) as *mut surface_t;
    stats.n = 0 as libc::c_int;
    stats.delaunay = 0 as *mut v_data;
    edgeStats(s, &mut stats);
    nsegs = stats.n;
    segs = gcalloc(
        (2 as libc::c_int * nsegs) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    state.n = 0 as libc::c_int;
    state.edges = segs;
    gts_surface_foreach_edge(
        s,
        Some(
            addEdge as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> gint,
        ),
        &mut state as *mut estate as gpointer,
    );
    gts_surface_foreach_face(
        s,
        Some(
            cntFace as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> gint,
        ),
        &mut nfaces as *mut libc::c_int as gpointer,
    );
    faces = gcalloc(
        (3 as libc::c_int * nfaces) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    neigh = gcalloc(
        (3 as libc::c_int * nfaces) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    statf.faces = faces;
    statf.neigh = neigh;
    gts_surface_foreach_face(
        s,
        Some(
            addFace as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> gint,
        ),
        &mut statf as *mut fstate as gpointer,
    );
    (*sf).nedges = nsegs;
    let ref mut fresh21 = (*sf).edges;
    *fresh21 = segs;
    (*sf).nfaces = nfaces;
    let ref mut fresh22 = (*sf).faces;
    *fresh22 = faces;
    let ref mut fresh23 = (*sf).neigh;
    *fresh23 = neigh;
    gts_object_destroy(s as *mut GtsObject);
    return sf;
}
#[no_mangle]
pub unsafe extern "C" fn get_triangles(
    mut x: *mut libc::c_double,
    mut n: libc::c_int,
    mut tris: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut s: *mut GtsSurface = 0 as *mut GtsSurface;
    let mut nfaces: libc::c_int = 0 as libc::c_int;
    let mut statf: fstate = fstate {
        s: 0 as *mut GtsSurface,
        faces: 0 as *mut libc::c_int,
        neigh: 0 as *mut libc::c_int,
    };
    if n <= 2 as libc::c_int {
        return 0 as *mut libc::c_int;
    }
    s = tri(
        x,
        0 as *mut libc::c_double,
        n,
        0 as *mut libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if s.is_null() {
        return 0 as *mut libc::c_int;
    }
    gts_surface_foreach_face(
        s,
        Some(
            cntFace as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> gint,
        ),
        &mut nfaces as *mut libc::c_int as gpointer,
    );
    statf
        .faces = gcalloc(
        (3 as libc::c_int * nfaces) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    gts_surface_foreach_face(
        s,
        Some(
            addTri as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> gint,
        ),
        &mut statf as *mut fstate as gpointer,
    );
    gts_object_destroy(s as *mut GtsObject);
    *tris = nfaces;
    return statf.faces;
}
#[no_mangle]
pub unsafe extern "C" fn freeSurface(mut s: *mut surface_t) {
    free((*s).edges as *mut libc::c_void);
    free((*s).faces as *mut libc::c_void);
    free((*s).neigh as *mut libc::c_void);
}
unsafe extern "C" fn remove_edge(
    mut graph: *mut v_data,
    mut source: libc::c_int,
    mut dest: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < (*graph.offset(source as isize)).nedges {
        if *((*graph.offset(source as isize)).edges).offset(i as isize) == dest {
            let ref mut fresh24 = (*graph.offset(source as isize)).nedges;
            *fresh24 -= 1;
            *((*graph.offset(source as isize)).edges)
                .offset(
                    i as isize,
                ) = *((*graph.offset(source as isize)).edges).offset(*fresh24 as isize);
            break;
        } else {
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn UG_graph(
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut n: libc::c_int,
    mut accurate_computation: libc::c_int,
) -> *mut v_data {
    let mut delaunay: *mut v_data = 0 as *mut v_data;
    let mut i: libc::c_int = 0;
    let mut dist_ij: libc::c_double = 0.;
    let mut dist_ik: libc::c_double = 0.;
    let mut dist_jk: libc::c_double = 0.;
    let mut x_i: libc::c_double = 0.;
    let mut y_i: libc::c_double = 0.;
    let mut x_j: libc::c_double = 0.;
    let mut y_j: libc::c_double = 0.;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut neighbor_j: libc::c_int = 0;
    let mut neighbor_k: libc::c_int = 0;
    if n == 2 as libc::c_int {
        let mut edges: *mut libc::c_int = gcalloc(
            4 as libc::c_int as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        delaunay = gcalloc(n as size_t, ::std::mem::size_of::<v_data>() as libc::c_ulong)
            as *mut v_data;
        let ref mut fresh25 = (*delaunay.offset(0 as libc::c_int as isize)).ewgts;
        *fresh25 = 0 as *mut libc::c_float;
        let ref mut fresh26 = (*delaunay.offset(0 as libc::c_int as isize)).edges;
        *fresh26 = edges;
        (*delaunay.offset(0 as libc::c_int as isize)).nedges = 2 as libc::c_int;
        *((*delaunay.offset(0 as libc::c_int as isize)).edges)
            .offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        *((*delaunay.offset(0 as libc::c_int as isize)).edges)
            .offset(1 as libc::c_int as isize) = 1 as libc::c_int;
        let ref mut fresh27 = (*delaunay.offset(1 as libc::c_int as isize)).edges;
        *fresh27 = edges.offset(2 as libc::c_int as isize);
        let ref mut fresh28 = (*delaunay.offset(1 as libc::c_int as isize)).ewgts;
        *fresh28 = 0 as *mut libc::c_float;
        (*delaunay.offset(1 as libc::c_int as isize)).nedges = 2 as libc::c_int;
        *((*delaunay.offset(1 as libc::c_int as isize)).edges)
            .offset(0 as libc::c_int as isize) = 1 as libc::c_int;
        *((*delaunay.offset(1 as libc::c_int as isize)).edges)
            .offset(1 as libc::c_int as isize) = 0 as libc::c_int;
        return delaunay;
    } else {
        if n == 1 as libc::c_int {
            let mut edges_0: *mut libc::c_int = gcalloc(
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ) as *mut libc::c_int;
            delaunay = gcalloc(
                n as size_t,
                ::std::mem::size_of::<v_data>() as libc::c_ulong,
            ) as *mut v_data;
            let ref mut fresh29 = (*delaunay.offset(0 as libc::c_int as isize)).ewgts;
            *fresh29 = 0 as *mut libc::c_float;
            let ref mut fresh30 = (*delaunay.offset(0 as libc::c_int as isize)).edges;
            *fresh30 = edges_0;
            (*delaunay.offset(0 as libc::c_int as isize)).nedges = 1 as libc::c_int;
            *((*delaunay.offset(0 as libc::c_int as isize)).edges)
                .offset(0 as libc::c_int as isize) = 0 as libc::c_int;
            return delaunay;
        }
    }
    delaunay = delaunay_triangulation(x, y, n);
    if accurate_computation != 0 {
        i = 0 as libc::c_int;
        while i < n {
            x_i = *x.offset(i as isize);
            y_i = *y.offset(i as isize);
            j = 1 as libc::c_int;
            while j < (*delaunay.offset(i as isize)).nedges {
                neighbor_j = *((*delaunay.offset(i as isize)).edges).offset(j as isize);
                if neighbor_j < i {
                    j += 1;
                } else {
                    x_j = *x.offset(neighbor_j as isize);
                    y_j = *y.offset(neighbor_j as isize);
                    dist_ij = (x_j - x_i) * (x_j - x_i) + (y_j - y_i) * (y_j - y_i);
                    let mut removed: bool = 0 as libc::c_int != 0;
                    k = 0 as libc::c_int;
                    while k < n && !removed {
                        dist_ik = (*x.offset(k as isize) - x_i)
                            * (*x.offset(k as isize) - x_i)
                            + (*y.offset(k as isize) - y_i)
                                * (*y.offset(k as isize) - y_i);
                        if dist_ik < dist_ij {
                            dist_jk = (*x.offset(k as isize) - x_j)
                                * (*x.offset(k as isize) - x_j)
                                + (*y.offset(k as isize) - y_j)
                                    * (*y.offset(k as isize) - y_j);
                            if dist_jk < dist_ij {
                                let ref mut fresh31 = (*delaunay.offset(i as isize)).nedges;
                                *fresh31 -= 1;
                                *((*delaunay.offset(i as isize)).edges)
                                    .offset(
                                        j as isize,
                                    ) = *((*delaunay.offset(i as isize)).edges)
                                    .offset(*fresh31 as isize);
                                remove_edge(delaunay, neighbor_j, i);
                                removed = 1 as libc::c_int != 0;
                            }
                        }
                        k += 1;
                    }
                    if !removed {
                        j += 1;
                    }
                }
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < n {
            x_i = *x.offset(i as isize);
            y_i = *y.offset(i as isize);
            j = 1 as libc::c_int;
            while j < (*delaunay.offset(i as isize)).nedges {
                neighbor_j = *((*delaunay.offset(i as isize)).edges).offset(j as isize);
                x_j = *x.offset(neighbor_j as isize);
                y_j = *y.offset(neighbor_j as isize);
                dist_ij = (x_j - x_i) * (x_j - x_i) + (y_j - y_i) * (y_j - y_i);
                let mut removed_0: bool = 0 as libc::c_int != 0;
                k = 1 as libc::c_int;
                while k < (*delaunay.offset(i as isize)).nedges && !removed_0 {
                    neighbor_k = *((*delaunay.offset(i as isize)).edges)
                        .offset(k as isize);
                    dist_ik = (*x.offset(neighbor_k as isize) - x_i)
                        * (*x.offset(neighbor_k as isize) - x_i)
                        + (*y.offset(neighbor_k as isize) - y_i)
                            * (*y.offset(neighbor_k as isize) - y_i);
                    if dist_ik < dist_ij {
                        dist_jk = (*x.offset(neighbor_k as isize) - x_j)
                            * (*x.offset(neighbor_k as isize) - x_j)
                            + (*y.offset(neighbor_k as isize) - y_j)
                                * (*y.offset(neighbor_k as isize) - y_j);
                        if dist_jk < dist_ij {
                            let ref mut fresh32 = (*delaunay.offset(i as isize)).nedges;
                            *fresh32 -= 1;
                            *((*delaunay.offset(i as isize)).edges)
                                .offset(
                                    j as isize,
                                ) = *((*delaunay.offset(i as isize)).edges)
                                .offset(*fresh32 as isize);
                            remove_edge(delaunay, neighbor_j, i);
                            removed_0 = 1 as libc::c_int != 0;
                        }
                    }
                    k += 1;
                }
                if !removed_0 {
                    j += 1;
                }
            }
            i += 1;
        }
    }
    return delaunay;
}
#[no_mangle]
pub unsafe extern "C" fn freeGraph(mut graph: *mut v_data) {
    if !graph.is_null() {
        free((*graph.offset(0 as libc::c_int as isize)).edges as *mut libc::c_void);
        free((*graph.offset(0 as libc::c_int as isize)).ewgts as *mut libc::c_void);
        free(graph as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn freeGraphData(mut graph: *mut vtx_data) {
    if !graph.is_null() {
        free((*graph.offset(0 as libc::c_int as isize)).edges as *mut libc::c_void);
        free((*graph.offset(0 as libc::c_int as isize)).ewgts as *mut libc::c_void);
        free((*graph.offset(0 as libc::c_int as isize)).edists as *mut libc::c_void);
        free(graph as *mut libc::c_void);
    }
}
