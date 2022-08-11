#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn bfs_bounded(
        _: libc::c_int,
        _: *mut vtx_data,
        _: libc::c_int,
        _: *mut DistType,
        _: *mut Queue,
        _: libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn freeQueue(_: *mut Queue);
    fn mkQueue(_: *mut Queue, _: libc::c_int);
}
pub type __uint8_t = libc::c_uchar;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type size_t = libc::c_ulong;
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
pub struct bitarray_t {
    pub c2rust_unnamed: C2RustUnnamed,
    pub size_bits: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub block: [uint8_t; 8],
    pub base: *mut uint8_t,
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
pub type DistType = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Queue {
    pub data: *mut libc::c_int,
    pub queueSize: libc::c_int,
    pub end: libc::c_int,
    pub start: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct term_sgd {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub d: libc::c_float,
    pub w: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct graph_sgd {
    pub n: size_t,
    pub sources: *mut size_t,
    pub pinneds: bitarray_t,
    pub targets: *mut size_t,
    pub weights: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heap {
    pub data: *mut libc::c_int,
    pub heapSize: libc::c_int,
}
pub type Word = DistType;
#[inline]
unsafe extern "C" fn bitarray_new(
    mut self_0: *mut bitarray_t,
    mut size_bits: size_t,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/bitarray.h\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"int bitarray_new(bitarray_t *, size_t)\0"))
                .as_ptr(),
        );
    }
    if (*self_0).size_bits == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"self->size_bits == 0\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/bitarray.h\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"int bitarray_new(bitarray_t *, size_t)\0"))
                .as_ptr(),
        );
    }
    if size_bits
        <= (::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        memset(
            ((*self_0).c2rust_unnamed.block).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
        );
    } else {
        let mut capacity: size_t = size_bits
            .wrapping_div(8 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (if size_bits.wrapping_rem(8 as libc::c_int as libc::c_ulong)
                    == 0 as libc::c_int as libc::c_ulong
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as libc::c_ulong,
            );
        let mut base: *mut uint8_t = calloc(
            capacity,
            ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
        ) as *mut uint8_t;
        if (base == 0 as *mut libc::c_void as *mut uint8_t) as libc::c_int
            as libc::c_long != 0
        {
            return 12 as libc::c_int;
        }
        let ref mut fresh0 = (*self_0).c2rust_unnamed.base;
        *fresh0 = base;
    }
    (*self_0).size_bits = size_bits;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn bitarray_new_or_exit(mut size_bits: size_t) -> bitarray_t {
    let mut ba: bitarray_t = bitarray_t {
        c2rust_unnamed: C2RustUnnamed { block: [0; 8] },
        size_bits: 0,
    };
    memset(
        &mut ba as *mut bitarray_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<bitarray_t>() as libc::c_ulong,
    );
    let mut error: libc::c_int = bitarray_new(&mut ba, size_bits);
    if (error != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    return ba;
}
#[inline]
unsafe extern "C" fn bitarray_get(mut self_0: bitarray_t, mut index: size_t) -> bool {
    if index < self_0.size_bits
        && !(b"out of bounds access\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"index < self.size_bits && \"out of bounds access\"\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/bitarray.h\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"_Bool bitarray_get(bitarray_t, size_t)\0"))
                .as_ptr(),
        );
    }
    let mut base: *const uint8_t = 0 as *const uint8_t;
    if self_0.size_bits
        <= (::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        base = (self_0.c2rust_unnamed.block).as_mut_ptr();
    } else {
        base = self_0.c2rust_unnamed.base;
    }
    return *base.offset(index.wrapping_div(8 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int >> index.wrapping_rem(8 as libc::c_int as libc::c_ulong)
        & 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn bitarray_set(
    mut self_0: *mut bitarray_t,
    mut index: size_t,
    mut value: bool,
) {
    if index < (*self_0).size_bits
        && !(b"out of bounds access\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"index < self->size_bits && \"out of bounds access\"\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/bitarray.h\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void bitarray_set(bitarray_t *, size_t, _Bool)\0"))
                .as_ptr(),
        );
    }
    let mut base: *mut uint8_t = 0 as *mut uint8_t;
    if (*self_0).size_bits
        <= (::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        base = ((*self_0).c2rust_unnamed.block).as_mut_ptr();
    } else {
        base = (*self_0).c2rust_unnamed.base;
    }
    if value {
        let ref mut fresh1 = *base
            .offset(index.wrapping_div(8 as libc::c_int as libc::c_ulong) as isize);
        *fresh1 = (*fresh1 as libc::c_int
            | ((1 as libc::c_int)
                << index.wrapping_rem(8 as libc::c_int as libc::c_ulong)) as uint8_t
                as libc::c_int) as uint8_t;
    } else {
        let ref mut fresh2 = *base
            .offset(index.wrapping_div(8 as libc::c_int as libc::c_ulong) as isize);
        *fresh2 = (*fresh2 as libc::c_int
            & !((1 as libc::c_int)
                << index.wrapping_rem(8 as libc::c_int as libc::c_ulong)) as uint8_t
                as libc::c_int) as uint8_t;
    };
}
#[inline]
unsafe extern "C" fn bitarray_reset(mut self_0: *mut bitarray_t) {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/bitarray.h\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void bitarray_reset(bitarray_t *)\0"))
                .as_ptr(),
        );
    }
    if (*self_0).size_bits
        > (::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        free((*self_0).c2rust_unnamed.base as *mut libc::c_void);
    }
    memset(
        self_0 as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<bitarray_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn heapify(
    mut h: *mut heap,
    mut i: libc::c_int,
    mut index: *mut libc::c_int,
    mut dist: *mut Word,
) {
    let mut l: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut largest: libc::c_int = 0;
    loop {
        l = 2 as libc::c_int * i;
        r = 2 as libc::c_int * i + 1 as libc::c_int;
        if l < (*h).heapSize
            && *dist.offset(*((*h).data).offset(l as isize) as isize)
                < *dist.offset(*((*h).data).offset(i as isize) as isize)
        {
            largest = l;
        } else {
            largest = i;
        }
        if r < (*h).heapSize
            && *dist.offset(*((*h).data).offset(r as isize) as isize)
                < *dist.offset(*((*h).data).offset(largest as isize) as isize)
        {
            largest = r;
        }
        if largest == i {
            break;
        }
        let mut temp: libc::c_int = 0;
        temp = *((*h).data).offset(largest as isize);
        *((*h).data).offset(largest as isize) = *((*h).data).offset(i as isize);
        *((*h).data).offset(i as isize) = temp;
        *index.offset(*((*h).data).offset(largest as isize) as isize) = largest;
        *index.offset(*((*h).data).offset(i as isize) as isize) = i;
        i = largest;
    };
}
unsafe extern "C" fn freeHeap(mut h: *mut heap) {
    free((*h).data as *mut libc::c_void);
}
unsafe extern "C" fn initHeap(
    mut h: *mut heap,
    mut startVertex: libc::c_int,
    mut index: *mut libc::c_int,
    mut dist: *mut Word,
    mut n: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if n == 1 as libc::c_int {
        let ref mut fresh3 = (*h).data;
        *fresh3 = 0 as *mut libc::c_int;
    } else {
        let ref mut fresh4 = (*h).data;
        *fresh4 = gcalloc(
            (n - 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
    }
    (*h).heapSize = n - 1 as libc::c_int;
    count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if i != startVertex {
            *((*h).data).offset(count as isize) = i;
            *index.offset(i as isize) = count;
            count += 1;
        }
        i += 1;
    }
    j = (n - 1 as libc::c_int) / 2 as libc::c_int;
    while j >= 0 as libc::c_int {
        heapify(h, j, index, dist);
        j -= 1;
    }
}
unsafe extern "C" fn extractMax(
    mut h: *mut heap,
    mut max: *mut libc::c_int,
    mut index: *mut libc::c_int,
    mut dist: *mut Word,
) -> bool {
    if (*h).heapSize == 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    *max = *((*h).data).offset(0 as libc::c_int as isize);
    *((*h).data)
        .offset(
            0 as libc::c_int as isize,
        ) = *((*h).data).offset(((*h).heapSize - 1 as libc::c_int) as isize);
    *index
        .offset(
            *((*h).data).offset(0 as libc::c_int as isize) as isize,
        ) = 0 as libc::c_int;
    let ref mut fresh5 = (*h).heapSize;
    *fresh5 -= 1;
    heapify(h, 0 as libc::c_int, index, dist);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn increaseKey(
    mut h: *mut heap,
    mut increasedVertex: libc::c_int,
    mut newDist: Word,
    mut index: *mut libc::c_int,
    mut dist: *mut Word,
) {
    let mut placeInHeap: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if *dist.offset(increasedVertex as isize) <= newDist {
        return;
    }
    placeInHeap = *index.offset(increasedVertex as isize);
    *dist.offset(increasedVertex as isize) = newDist;
    i = placeInHeap;
    while i > 0 as libc::c_int
        && *dist.offset(*((*h).data).offset((i / 2 as libc::c_int) as isize) as isize)
            > newDist
    {
        *((*h).data)
            .offset(i as isize) = *((*h).data).offset((i / 2 as libc::c_int) as isize);
        *index.offset(*((*h).data).offset(i as isize) as isize) = i;
        i = i / 2 as libc::c_int;
    }
    *((*h).data).offset(i as isize) = increasedVertex;
    *index.offset(increasedVertex as isize) = i;
}
#[no_mangle]
pub unsafe extern "C" fn dijkstra(
    mut vertex: libc::c_int,
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut dist: *mut DistType,
) {
    let mut i: libc::c_int = 0;
    let mut H: heap = heap {
        data: 0 as *mut libc::c_int,
        heapSize: 0,
    };
    let mut closestVertex: libc::c_int = 0;
    let mut neighbor: libc::c_int = 0;
    let mut closestDist: DistType = 0;
    let mut prevClosestDist: DistType = 2147483647 as libc::c_int;
    let mut index: *mut libc::c_int = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *dist.offset(i as isize) = 2147483647 as libc::c_int;
        i += 1;
    }
    *dist.offset(vertex as isize) = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < (*graph.offset(vertex as isize)).nedges {
        *dist
            .offset(
                *((*graph.offset(vertex as isize)).edges).offset(i as isize) as isize,
            ) = *((*graph.offset(vertex as isize)).ewgts).offset(i as isize) as DistType;
        i += 1;
    }
    initHeap(&mut H, vertex, index, dist, n);
    while extractMax(&mut H, &mut closestVertex, index, dist) {
        closestDist = *dist.offset(closestVertex as isize);
        if closestDist == 2147483647 as libc::c_int {
            break;
        }
        i = 1 as libc::c_int;
        while i < (*graph.offset(closestVertex as isize)).nedges {
            neighbor = *((*graph.offset(closestVertex as isize)).edges)
                .offset(i as isize);
            increaseKey(
                &mut H,
                neighbor,
                closestDist
                    + *((*graph.offset(closestVertex as isize)).ewgts).offset(i as isize)
                        as DistType,
                index,
                dist,
            );
            i += 1;
        }
        prevClosestDist = closestDist;
    }
    i = 0 as libc::c_int;
    while i < n {
        if *dist.offset(i as isize) == 2147483647 as libc::c_int {
            *dist.offset(i as isize) = prevClosestDist + 10 as libc::c_int;
        }
        i += 1;
    }
    freeHeap(&mut H);
    free(index as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn dijkstra_bounded(
    mut vertex: libc::c_int,
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut dist: *mut DistType,
    mut bound: libc::c_int,
    mut visited_nodes: *mut libc::c_int,
) -> libc::c_int {
    let mut num_visited_nodes: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut Q: Queue = Queue {
        data: 0 as *mut libc::c_int,
        queueSize: 0,
        end: 0,
        start: 0,
    };
    let mut H: heap = heap {
        data: 0 as *mut libc::c_int,
        heapSize: 0,
    };
    let mut closestVertex: libc::c_int = 0;
    let mut neighbor: libc::c_int = 0;
    let mut closestDist: DistType = 0;
    let mut num_found: libc::c_int = 0 as libc::c_int;
    mkQueue(&mut Q, n);
    i = 0 as libc::c_int;
    while i < n {
        *dist.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    num_visited_nodes = bfs_bounded(
        vertex,
        graph,
        n,
        dist,
        &mut Q,
        bound,
        visited_nodes,
    );
    let mut node_in_neighborhood: bitarray_t = bitarray_new_or_exit(n as size_t);
    i = 0 as libc::c_int;
    while i < num_visited_nodes {
        bitarray_set(
            &mut node_in_neighborhood,
            *visited_nodes.offset(i as isize) as size_t,
            1 as libc::c_int != 0,
        );
        i += 1;
    }
    let mut index: *mut libc::c_int = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *dist.offset(i as isize) = 2147483647 as libc::c_int;
        i += 1;
    }
    *dist.offset(vertex as isize) = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < (*graph.offset(vertex as isize)).nedges {
        *dist
            .offset(
                *((*graph.offset(vertex as isize)).edges).offset(i as isize) as isize,
            ) = *((*graph.offset(vertex as isize)).ewgts).offset(i as isize) as DistType;
        i += 1;
    }
    initHeap(&mut H, vertex, index, dist, n);
    while num_found < num_visited_nodes
        && extractMax(&mut H, &mut closestVertex, index, dist) as libc::c_int != 0
    {
        if bitarray_get(node_in_neighborhood, closestVertex as size_t) {
            num_found += 1;
        }
        closestDist = *dist.offset(closestVertex as isize);
        if closestDist == 2147483647 as libc::c_int {
            break;
        }
        i = 1 as libc::c_int;
        while i < (*graph.offset(closestVertex as isize)).nedges {
            neighbor = *((*graph.offset(closestVertex as isize)).edges)
                .offset(i as isize);
            increaseKey(
                &mut H,
                neighbor,
                closestDist
                    + *((*graph.offset(closestVertex as isize)).ewgts).offset(i as isize)
                        as DistType,
                index,
                dist,
            );
            i += 1;
        }
    }
    bitarray_reset(&mut node_in_neighborhood);
    freeHeap(&mut H);
    free(index as *mut libc::c_void);
    freeQueue(&mut Q);
    return num_visited_nodes;
}
unsafe extern "C" fn heapify_f(
    mut h: *mut heap,
    mut i: libc::c_int,
    mut index: *mut libc::c_int,
    mut dist: *mut libc::c_float,
) {
    let mut l: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut largest: libc::c_int = 0;
    loop {
        l = 2 as libc::c_int * i;
        r = 2 as libc::c_int * i + 1 as libc::c_int;
        if l < (*h).heapSize
            && *dist.offset(*((*h).data).offset(l as isize) as isize)
                < *dist.offset(*((*h).data).offset(i as isize) as isize)
        {
            largest = l;
        } else {
            largest = i;
        }
        if r < (*h).heapSize
            && *dist.offset(*((*h).data).offset(r as isize) as isize)
                < *dist.offset(*((*h).data).offset(largest as isize) as isize)
        {
            largest = r;
        }
        if largest == i {
            break;
        }
        let mut temp: libc::c_int = 0;
        temp = *((*h).data).offset(largest as isize);
        *((*h).data).offset(largest as isize) = *((*h).data).offset(i as isize);
        *((*h).data).offset(i as isize) = temp;
        *index.offset(*((*h).data).offset(largest as isize) as isize) = largest;
        *index.offset(*((*h).data).offset(i as isize) as isize) = i;
        i = largest;
    };
}
unsafe extern "C" fn initHeap_f(
    mut h: *mut heap,
    mut startVertex: libc::c_int,
    mut index: *mut libc::c_int,
    mut dist: *mut libc::c_float,
    mut n: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let ref mut fresh6 = (*h).data;
    *fresh6 = gcalloc(
        (n - 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    (*h).heapSize = n - 1 as libc::c_int;
    count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if i != startVertex {
            *((*h).data).offset(count as isize) = i;
            *index.offset(i as isize) = count;
            count += 1;
        }
        i += 1;
    }
    j = (n - 1 as libc::c_int) / 2 as libc::c_int;
    while j >= 0 as libc::c_int {
        heapify_f(h, j, index, dist);
        j -= 1;
    }
}
unsafe extern "C" fn extractMax_f(
    mut h: *mut heap,
    mut max: *mut libc::c_int,
    mut index: *mut libc::c_int,
    mut dist: *mut libc::c_float,
) -> bool {
    if (*h).heapSize == 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    *max = *((*h).data).offset(0 as libc::c_int as isize);
    *((*h).data)
        .offset(
            0 as libc::c_int as isize,
        ) = *((*h).data).offset(((*h).heapSize - 1 as libc::c_int) as isize);
    *index
        .offset(
            *((*h).data).offset(0 as libc::c_int as isize) as isize,
        ) = 0 as libc::c_int;
    let ref mut fresh7 = (*h).heapSize;
    *fresh7 -= 1;
    heapify_f(h, 0 as libc::c_int, index, dist);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn increaseKey_f(
    mut h: *mut heap,
    mut increasedVertex: libc::c_int,
    mut newDist: libc::c_float,
    mut index: *mut libc::c_int,
    mut dist: *mut libc::c_float,
) {
    let mut placeInHeap: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if *dist.offset(increasedVertex as isize) <= newDist {
        return;
    }
    placeInHeap = *index.offset(increasedVertex as isize);
    *dist.offset(increasedVertex as isize) = newDist;
    i = placeInHeap;
    while i > 0 as libc::c_int
        && *dist.offset(*((*h).data).offset((i / 2 as libc::c_int) as isize) as isize)
            > newDist
    {
        *((*h).data)
            .offset(i as isize) = *((*h).data).offset((i / 2 as libc::c_int) as isize);
        *index.offset(*((*h).data).offset(i as isize) as isize) = i;
        i = i / 2 as libc::c_int;
    }
    *((*h).data).offset(i as isize) = increasedVertex;
    *index.offset(increasedVertex as isize) = i;
}
#[no_mangle]
pub unsafe extern "C" fn dijkstra_f(
    mut vertex: libc::c_int,
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut dist: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut H: heap = heap {
        data: 0 as *mut libc::c_int,
        heapSize: 0,
    };
    let mut closestVertex: libc::c_int = 0 as libc::c_int;
    let mut neighbor: libc::c_int = 0;
    let mut closestDist: libc::c_float = 0.;
    let mut index: *mut libc::c_int = 0 as *mut libc::c_int;
    index = gcalloc(n as size_t, ::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *dist.offset(i as isize) = 3.40282347e+38f32;
        i += 1;
    }
    *dist.offset(vertex as isize) = 0 as libc::c_int as libc::c_float;
    i = 1 as libc::c_int;
    while i < (*graph.offset(vertex as isize)).nedges {
        *dist
            .offset(
                *((*graph.offset(vertex as isize)).edges).offset(i as isize) as isize,
            ) = *((*graph.offset(vertex as isize)).ewgts).offset(i as isize);
        i += 1;
    }
    initHeap_f(&mut H, vertex, index, dist, n);
    while extractMax_f(&mut H, &mut closestVertex, index, dist) {
        closestDist = *dist.offset(closestVertex as isize);
        if closestDist == 3.40282347e+38f32 {
            break;
        }
        i = 1 as libc::c_int;
        while i < (*graph.offset(closestVertex as isize)).nedges {
            neighbor = *((*graph.offset(closestVertex as isize)).edges)
                .offset(i as isize);
            increaseKey_f(
                &mut H,
                neighbor,
                closestDist
                    + *((*graph.offset(closestVertex as isize)).ewgts)
                        .offset(i as isize),
                index,
                dist,
            );
            i += 1;
        }
    }
    freeHeap(&mut H);
    free(index as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn dijkstra_sgd(
    mut graph: *mut graph_sgd,
    mut source: libc::c_int,
    mut terms: *mut term_sgd,
) -> libc::c_int {
    let mut h: heap = heap {
        data: 0 as *mut libc::c_int,
        heapSize: 0,
    };
    let mut indices: *mut libc::c_int = gcalloc(
        (*graph).n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut dists: *mut libc::c_float = gcalloc(
        (*graph).n,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*graph).n {
        *dists.offset(i as isize) = 3.40282347e+38f32;
        i = i.wrapping_add(1);
    }
    *dists.offset(source as isize) = 0 as libc::c_int as libc::c_float;
    let mut i_0: size_t = *((*graph).sources).offset(source as isize);
    while i_0 < *((*graph).sources).offset((source + 1 as libc::c_int) as isize) {
        let mut target: size_t = *((*graph).targets).offset(i_0 as isize);
        *dists.offset(target as isize) = *((*graph).weights).offset(i_0 as isize);
        i_0 = i_0.wrapping_add(1);
    }
    if (*graph).n <= 2147483647 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"graph->n <= INT_MAX\0" as *const u8 as *const libc::c_char,
            b"dijkstra.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"int dijkstra_sgd(graph_sgd *, int, term_sgd *)\0"))
                .as_ptr(),
        );
    }
    initHeap_f(&mut h, source, indices, dists, (*graph).n as libc::c_int);
    let mut closest: libc::c_int = 0 as libc::c_int;
    let mut offset: libc::c_int = 0 as libc::c_int;
    while extractMax_f(&mut h, &mut closest, indices, dists) {
        let mut d: libc::c_float = *dists.offset(closest as isize);
        if d == 3.40282347e+38f32 {
            break;
        }
        if bitarray_get((*graph).pinneds, closest as size_t) as libc::c_int != 0
            || closest < source
        {
            (*terms.offset(offset as isize)).i = source;
            (*terms.offset(offset as isize)).j = closest;
            (*terms.offset(offset as isize)).d = d;
            (*terms.offset(offset as isize))
                .w = 1 as libc::c_int as libc::c_float / (d * d);
            offset += 1;
        }
        let mut i_1: size_t = *((*graph).sources).offset(closest as isize);
        while i_1 < *((*graph).sources).offset((closest + 1 as libc::c_int) as isize) {
            let mut target_0: size_t = *((*graph).targets).offset(i_1 as isize);
            let mut weight: libc::c_float = *((*graph).weights).offset(i_1 as isize);
            if target_0 <= 2147483647 as libc::c_int as size_t {} else {
                __assert_fail(
                    b"target <= (size_t)INT_MAX\0" as *const u8 as *const libc::c_char,
                    b"dijkstra.c\0" as *const u8 as *const libc::c_char,
                    394 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 47],
                        &[libc::c_char; 47],
                    >(b"int dijkstra_sgd(graph_sgd *, int, term_sgd *)\0"))
                        .as_ptr(),
                );
            }
            increaseKey_f(&mut h, target_0 as libc::c_int, d + weight, indices, dists);
            i_1 = i_1.wrapping_add(1);
        }
    }
    freeHeap(&mut h);
    free(indices as *mut libc::c_void);
    free(dists as *mut libc::c_void);
    return offset;
}
