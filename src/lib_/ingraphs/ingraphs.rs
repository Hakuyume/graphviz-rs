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
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn agsetfile(_: *const libc::c_char);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agraph_t {
    pub dummy: *mut libc::c_char,
}
pub type opengfn = Option<unsafe extern "C" fn(*mut FILE) -> *mut Agraph_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ingdisc {
    pub openf: Option<unsafe extern "C" fn(*mut libc::c_char) -> *mut libc::c_void>,
    pub readf: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut Agraph_t>,
    pub closef: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub dflt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ingraph_state {
    pub u: C2RustUnnamed,
    pub ctr: libc::c_int,
    pub ingraphs: libc::c_int,
    pub fp: *mut libc::c_void,
    pub fns: *mut ingdisc,
    pub heap: bool,
    pub errors: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub Files: *mut *mut libc::c_char,
    pub Graphs: *mut *mut Agraph_t,
}
pub type xopengfn = Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut Agraph_t>;
unsafe extern "C" fn nextFile(mut sp: *mut ingraph_state) {
    let mut rv: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*sp).u.Files).is_null() {
        let ref mut fresh0 = (*sp).ctr;
        let fresh1 = *fresh0;
        *fresh0 = *fresh0 + 1;
        if fresh1 == 0 as libc::c_int {
            rv = (*(*sp).fns).dflt;
        }
    } else {
        loop {
            let ref mut fresh2 = (*sp).ctr;
            let fresh3 = *fresh2;
            *fresh2 = *fresh2 + 1;
            fname = *((*sp).u.Files).offset(fresh3 as isize);
            if fname.is_null() {
                break;
            }
            if *fname as libc::c_int == '-' as i32 {
                rv = (*(*sp).fns).dflt;
                break;
            } else {
                rv = ((*(*sp).fns).openf).expect("non-null function pointer")(fname);
                if !rv.is_null() {
                    break;
                }
                fprintf(
                    stderr,
                    b"Can't open %s\n\0" as *const u8 as *const libc::c_char,
                    *((*sp).u.Files).offset(((*sp).ctr - 1 as libc::c_int) as isize),
                );
                let ref mut fresh4 = (*sp).errors;
                *fresh4 = (*fresh4).wrapping_add(1);
            }
        }
    }
    if !rv.is_null() {
        agsetfile(fileName(sp));
    }
    let ref mut fresh5 = (*sp).fp;
    *fresh5 = rv;
}
#[no_mangle]
pub unsafe extern "C" fn nextGraph(mut sp: *mut ingraph_state) -> *mut Agraph_t {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    if (*sp).ingraphs != 0 {
        g = *((*sp).u.Graphs).offset((*sp).ctr as isize);
        if !g.is_null() {
            let ref mut fresh6 = (*sp).ctr;
            *fresh6 += 1;
        }
        return g;
    }
    if ((*sp).fp).is_null() {
        nextFile(sp);
    }
    g = 0 as *mut Agraph_t;
    while !((*sp).fp).is_null() {
        g = ((*(*sp).fns).readf).expect("non-null function pointer")((*sp).fp);
        if !g.is_null() {
            break;
        }
        if !((*sp).u.Files).is_null() {
            ((*(*sp).fns).closef).expect("non-null function pointer")((*sp).fp);
        }
        nextFile(sp);
    }
    return g;
}
unsafe extern "C" fn new_ing(
    mut sp: *mut ingraph_state,
    mut files: *mut *mut libc::c_char,
    mut graphs: *mut *mut Agraph_t,
    mut disc: *mut ingdisc,
) -> *mut ingraph_state {
    if sp.is_null() {
        sp = malloc(::std::mem::size_of::<ingraph_state>() as libc::c_ulong) as *mut ingraph_state;
        if sp.is_null() {
            fprintf(
                stderr,
                b"ingraphs: out of memory\n\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut ingraph_state;
        }
        (*sp).heap = 1 as libc::c_int != 0;
    } else {
        (*sp).heap = 0 as libc::c_int != 0;
    }
    if !graphs.is_null() {
        (*sp).ingraphs = 1 as libc::c_int;
        let ref mut fresh7 = (*sp).u.Graphs;
        *fresh7 = graphs;
    } else {
        (*sp).ingraphs = 0 as libc::c_int;
        let ref mut fresh8 = (*sp).u.Files;
        *fresh8 = files;
    }
    (*sp).ctr = 0 as libc::c_int;
    (*sp).errors = 0 as libc::c_int as libc::c_uint;
    let ref mut fresh9 = (*sp).fp;
    *fresh9 = 0 as *mut libc::c_void;
    let ref mut fresh10 = (*sp).fns;
    *fresh10 = malloc(::std::mem::size_of::<ingdisc>() as libc::c_ulong) as *mut ingdisc;
    if ((*sp).fns).is_null() {
        fprintf(
            stderr,
            b"ingraphs: out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        if (*sp).heap {
            free(sp as *mut libc::c_void);
        }
        return 0 as *mut ingraph_state;
    }
    if ((*disc).openf).is_none()
        || ((*disc).readf).is_none()
        || ((*disc).closef).is_none()
        || ((*disc).dflt).is_null()
    {
        free((*sp).fns as *mut libc::c_void);
        if (*sp).heap {
            free(sp as *mut libc::c_void);
        }
        fprintf(
            stderr,
            b"ingraphs: NULL field in ingdisc argument\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut ingraph_state;
    }
    *(*sp).fns = *disc;
    return sp;
}
#[no_mangle]
pub unsafe extern "C" fn newIng(
    mut sp: *mut ingraph_state,
    mut files: *mut *mut libc::c_char,
    mut disc: *mut ingdisc,
) -> *mut ingraph_state {
    return new_ing(sp, files, 0 as *mut *mut Agraph_t, disc);
}
#[no_mangle]
pub unsafe extern "C" fn newIngGraphs(
    mut sp: *mut ingraph_state,
    mut graphs: *mut *mut Agraph_t,
    mut disc: *mut ingdisc,
) -> *mut ingraph_state {
    return new_ing(sp, 0 as *mut *mut libc::c_char, graphs, disc);
}
unsafe extern "C" fn dflt_open(mut f: *mut libc::c_char) -> *mut libc::c_void {
    return fopen(f, b"r\0" as *const u8 as *const libc::c_char) as *mut libc::c_void;
}
unsafe extern "C" fn dflt_close(mut fp: *mut libc::c_void) -> libc::c_int {
    return fclose(fp as *mut FILE);
}
static mut dflt_disc: ingdisc = unsafe {
    {
        let mut init = ingdisc {
            openf: Some(dflt_open as unsafe extern "C" fn(*mut libc::c_char) -> *mut libc::c_void),
            readf: None,
            closef: Some(dflt_close as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int),
            dflt: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn newIngraph(
    mut sp: *mut ingraph_state,
    mut files: *mut *mut libc::c_char,
    mut opf: opengfn,
) -> *mut ingraph_state {
    if (dflt_disc.dflt).is_null() {
        dflt_disc.dflt = stdin as *mut libc::c_void;
    }
    if opf.is_some() {
        dflt_disc.readf = ::std::mem::transmute::<opengfn, xopengfn>(opf);
    } else {
        fprintf(
            stderr,
            b"ingraphs: NULL graph reader\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut ingraph_state;
    }
    return newIng(sp, files, &mut dflt_disc);
}
#[no_mangle]
pub unsafe extern "C" fn closeIngraph(mut sp: *mut ingraph_state) {
    if (*sp).ingraphs == 0 && !((*sp).u.Files).is_null() && !((*sp).fp).is_null() {
        ((*(*sp).fns).closef).expect("non-null function pointer")((*sp).fp);
    }
    free((*sp).fns as *mut libc::c_void);
    if (*sp).heap {
        free(sp as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fileName(mut sp: *mut ingraph_state) -> *mut libc::c_char {
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*sp).ingraphs != 0 {
        return b"<>\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else if !((*sp).u.Files).is_null() {
        if (*sp).ctr != 0 {
            fname = *((*sp).u.Files).offset(((*sp).ctr - 1 as libc::c_int) as isize);
            if *fname as libc::c_int == '-' as i32 {
                return b"<stdin>\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            } else {
                return fname;
            }
        } else {
            return b"<>\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    } else {
        return b"<stdin>\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    };
}
