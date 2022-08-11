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
    pub type cell;
    fn free(_: *mut libc::c_void);
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
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
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snode {
    pub n_val: libc::c_int,
    pub n_idx: libc::c_int,
    pub n_dad: *mut snode,
    pub n_edge: *mut sedge,
    pub n_adj: libc::c_short,
    pub save_n_adj: libc::c_short,
    pub cells: [*mut cell; 2],
    pub adj_edge_list: *mut libc::c_int,
    pub index: libc::c_int,
    pub isVert: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sedge {
    pub weight: libc::c_double,
    pub cnt: libc::c_int,
    pub v1: libc::c_int,
    pub v2: libc::c_int,
}
static mut pq: *mut *mut snode = 0 as *const *mut snode as *mut *mut snode;
static mut PQcnt: libc::c_int = 0;
static mut guard: snode = snode {
    n_val: 0,
    n_idx: 0,
    n_dad: 0 as *const snode as *mut snode,
    n_edge: 0 as *const sedge as *mut sedge,
    n_adj: 0,
    save_n_adj: 0,
    cells: [0 as *const cell as *mut cell; 2],
    adj_edge_list: 0 as *const libc::c_int as *mut libc::c_int,
    index: 0,
    isVert: false,
};
static mut PQsize: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn PQgen(mut sz: libc::c_int) {
    if pq.is_null() {
        pq = gcalloc(
            (sz + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut snode>() as libc::c_ulong,
        ) as *mut *mut snode;
        let ref mut fresh0 = *pq.offset(0 as libc::c_int as isize);
        *fresh0 = &mut guard;
        PQsize = sz;
    }
    PQcnt = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PQfree() {
    free(pq as *mut libc::c_void);
    pq = 0 as *mut *mut snode;
    PQcnt = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PQinit() {
    PQcnt = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PQcheck() {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= PQcnt {
        if (**pq.offset(i as isize)).n_idx != i {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"fPQ.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"void PQcheck(void)\0"))
                    .as_ptr(),
            );
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQupheap(mut k: libc::c_int) {
    let mut x: *mut snode = *pq.offset(k as isize);
    let mut v: libc::c_int = (*x).n_val;
    let mut next: libc::c_int = k / 2 as libc::c_int;
    let mut n: *mut snode = 0 as *mut snode;
    loop {
        n = *pq.offset(next as isize);
        if !((*n).n_val < v) {
            break;
        }
        let ref mut fresh1 = *pq.offset(k as isize);
        *fresh1 = n;
        (*n).n_idx = k;
        k = next;
        next /= 2 as libc::c_int;
    }
    let ref mut fresh2 = *pq.offset(k as isize);
    *fresh2 = x;
    (*x).n_idx = k;
}
#[no_mangle]
pub unsafe extern "C" fn PQ_insert(mut np: *mut snode) -> libc::c_int {
    if PQcnt == PQsize {
        agerr(
            AGERR,
            b"Heap overflow\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    PQcnt += 1;
    let ref mut fresh3 = *pq.offset(PQcnt as isize);
    *fresh3 = np;
    PQupheap(PQcnt);
    PQcheck();
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PQdownheap(mut k: libc::c_int) {
    let mut x: *mut snode = *pq.offset(k as isize);
    let mut v: libc::c_int = (*x).n_val;
    let mut lim: libc::c_int = PQcnt / 2 as libc::c_int;
    let mut n: *mut snode = 0 as *mut snode;
    let mut j: libc::c_int = 0;
    while k <= lim {
        j = k + k;
        n = *pq.offset(j as isize);
        if j < PQcnt {
            if (*n).n_val < (**pq.offset((j + 1 as libc::c_int) as isize)).n_val {
                j += 1;
                n = *pq.offset(j as isize);
            }
        }
        if v >= (*n).n_val {
            break;
        }
        let ref mut fresh4 = *pq.offset(k as isize);
        *fresh4 = n;
        (*n).n_idx = k;
        k = j;
    }
    let ref mut fresh5 = *pq.offset(k as isize);
    *fresh5 = x;
    (*x).n_idx = k;
}
#[no_mangle]
pub unsafe extern "C" fn PQremove() -> *mut snode {
    let mut n: *mut snode = 0 as *mut snode;
    if PQcnt != 0 {
        n = *pq.offset(1 as libc::c_int as isize);
        let ref mut fresh6 = *pq.offset(1 as libc::c_int as isize);
        *fresh6 = *pq.offset(PQcnt as isize);
        PQcnt -= 1;
        if PQcnt != 0 {
            PQdownheap(1 as libc::c_int);
        }
        PQcheck();
        return n;
    } else {
        return 0 as *mut snode;
    };
}
#[no_mangle]
pub unsafe extern "C" fn PQupdate(mut n: *mut snode, mut d: libc::c_int) {
    (*n).n_val = d;
    PQupheap((*n).n_idx);
    PQcheck();
}
#[no_mangle]
pub unsafe extern "C" fn PQprint() {
    let mut i: libc::c_int = 0;
    let mut n: *mut snode = 0 as *mut snode;
    fprintf(stderr, b"Q: \0" as *const u8 as *const libc::c_char);
    i = 1 as libc::c_int;
    while i <= PQcnt {
        n = *pq.offset(i as isize);
        fprintf(
            stderr,
            b"%d(%d:%d) \0" as *const u8 as *const libc::c_char,
            (*n).index,
            (*n).n_idx,
            (*n).n_val,
        );
        i += 1;
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
