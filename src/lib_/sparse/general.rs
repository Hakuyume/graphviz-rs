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
#![feature(label_break_value, register_tool)]
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn rand() -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
#[no_mangle]
pub unsafe extern "C" fn drand() -> libc::c_double {
    return rand() as libc::c_double / 2147483647 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn irand(mut n: libc::c_int) -> libc::c_int {
    if n > 1 as libc::c_int {
    } else {
        __assert_fail(
            b"n > 1\0" as *const u8 as *const libc::c_char,
            b"general.c\0" as *const u8 as *const libc::c_char,
            26 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"int irand(int)\0"))
                .as_ptr(),
        );
    }
    return rand() % n;
}
#[no_mangle]
pub unsafe extern "C" fn random_permutation(mut n: libc::c_int) -> *mut libc::c_int {
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pp: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if n <= 0 as libc::c_int {
        return 0 as *mut libc::c_int;
    }
    p = gmalloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *p.offset(i as isize) = i;
        i += 1;
    }
    len = n;
    while len > 1 as libc::c_int {
        j = irand(len);
        pp = *p.offset((len - 1 as libc::c_int) as isize);
        *p.offset((len - 1 as libc::c_int) as isize) = *p.offset(j as isize);
        *p.offset(j as isize) = pp;
        len -= 1;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn vector_subtract_to(
    mut n: libc::c_int,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) -> *mut libc::c_double {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *y.offset(i as isize) = *x.offset(i as isize) - *y.offset(i as isize);
        i += 1;
    }
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn vector_product(
    mut n: libc::c_int,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) -> libc::c_double {
    let mut res: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        res += *x.offset(i as isize) * *y.offset(i as isize);
        i += 1;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn vector_saxpy(
    mut n: libc::c_int,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut beta: libc::c_double,
) -> *mut libc::c_double {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *y.offset(i as isize) = *x.offset(i as isize) + beta * *y.offset(i as isize);
        i += 1;
    }
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn vector_saxpy2(
    mut n: libc::c_int,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut beta: libc::c_double,
) -> *mut libc::c_double {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *x.offset(i as isize) = *x.offset(i as isize) + beta * *y.offset(i as isize);
        i += 1;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn vector_print(
    mut s: *mut libc::c_char,
    mut n: libc::c_int,
    mut x: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    printf(b"%s{\0" as *const u8 as *const libc::c_char, s);
    i = 0 as libc::c_int;
    while i < n {
        if i > 0 as libc::c_int {
            printf(b",\0" as *const u8 as *const libc::c_char);
        }
        printf(
            b"%f\0" as *const u8 as *const libc::c_char,
            *x.offset(i as isize),
        );
        i += 1;
    }
    printf(b"}\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn vector_float_take(
    mut n: libc::c_int,
    mut v: *mut libc::c_float,
    mut m: libc::c_int,
    mut p: *mut libc::c_int,
    mut u: *mut *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    if (*u).is_null() {
        *u = gmalloc(
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(m as libc::c_ulong),
        ) as *mut libc::c_float;
    }
    i = 0 as libc::c_int;
    while i < m {
        if *p.offset(i as isize) < n && *p.offset(i as isize) >= 0 as libc::c_int {
        } else {
            __assert_fail(
                b"p[i] < n && p[i] >= 0\0" as *const u8 as *const libc::c_char,
                b"general.c\0" as *const u8 as *const libc::c_char,
                94 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                    b"void vector_float_take(int, float *, int, int *, float **)\0",
                ))
                .as_ptr(),
            );
        }
        *(*u).offset(i as isize) = *v.offset(*p.offset(i as isize) as isize);
        i += 1;
    }
}
unsafe extern "C" fn comp_ascend(
    mut s1: *const libc::c_void,
    mut s2: *const libc::c_void,
) -> libc::c_int {
    let mut ss1: *const libc::c_double = s1 as *const libc::c_double;
    let mut ss2: *const libc::c_double = s2 as *const libc::c_double;
    if *ss1.offset(0 as libc::c_int as isize) > *ss2.offset(0 as libc::c_int as isize) {
        return 1 as libc::c_int;
    } else {
        if *ss1.offset(0 as libc::c_int as isize) < *ss2.offset(0 as libc::c_int as isize) {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn comp_ascend_int(
    mut s1: *const libc::c_void,
    mut s2: *const libc::c_void,
) -> libc::c_int {
    let mut ss1: *const libc::c_int = s1 as *const libc::c_int;
    let mut ss2: *const libc::c_int = s2 as *const libc::c_int;
    if *ss1.offset(0 as libc::c_int as isize) > *ss2.offset(0 as libc::c_int as isize) {
        return 1 as libc::c_int;
    } else {
        if *ss1.offset(0 as libc::c_int as isize) < *ss2.offset(0 as libc::c_int as isize) {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vector_ordering(
    mut n: libc::c_int,
    mut v: *mut libc::c_double,
    mut p: *mut *mut libc::c_int,
) {
    let mut u: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    if (*p).is_null() {
        *p = gmalloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut libc::c_int;
    }
    u = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        *u.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize) = i as libc::c_double;
        *u.offset((2 as libc::c_int * i) as isize) = *v.offset(i as isize);
        i += 1;
    }
    qsort(
        u as *mut libc::c_void,
        n as size_t,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        Some(
            comp_ascend
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < n {
        *(*p).offset(i as isize) =
            *u.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize) as libc::c_int;
        i += 1;
    }
    free(u as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn vector_sort_int(mut n: libc::c_int, mut v: *mut libc::c_int) {
    qsort(
        v as *mut libc::c_void,
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        Some(
            comp_ascend_int
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn distance_cropped(
    mut x: *mut libc::c_double,
    mut dim: libc::c_int,
    mut i: libc::c_int,
    mut j: libc::c_int,
) -> libc::c_double {
    let mut k: libc::c_int = 0;
    let mut dist: libc::c_double = 0.0f64;
    k = 0 as libc::c_int;
    while k < dim {
        dist += (*x.offset((i * dim + k) as isize) - *x.offset((j * dim + k) as isize))
            * (*x.offset((i * dim + k) as isize) - *x.offset((j * dim + k) as isize));
        k += 1;
    }
    dist = sqrt(dist);
    return if dist > 1.0e-15f64 { dist } else { 1.0e-15f64 };
}
#[no_mangle]
pub unsafe extern "C" fn distance(
    mut x: *mut libc::c_double,
    mut dim: libc::c_int,
    mut i: libc::c_int,
    mut j: libc::c_int,
) -> libc::c_double {
    let mut k: libc::c_int = 0;
    let mut dist: libc::c_double = 0.0f64;
    k = 0 as libc::c_int;
    while k < dim {
        dist += (*x.offset((i * dim + k) as isize) - *x.offset((j * dim + k) as isize))
            * (*x.offset((i * dim + k) as isize) - *x.offset((j * dim + k) as isize));
        k += 1;
    }
    dist = sqrt(dist);
    return dist;
}
#[no_mangle]
pub unsafe extern "C" fn point_distance(
    mut p1: *mut libc::c_double,
    mut p2: *mut libc::c_double,
    mut dim: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut dist: libc::c_double = 0.;
    dist = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < dim {
        dist += (*p1.offset(i as isize) - *p2.offset(i as isize))
            * (*p1.offset(i as isize) - *p2.offset(i as isize));
        i += 1;
    }
    return sqrt(dist);
}
#[no_mangle]
pub unsafe extern "C" fn strip_dir(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut first: bool = 1 as libc::c_int != 0;
    if s.is_null() {
        return s;
    }
    let mut i: size_t = strlen(s);
    loop {
        if first as libc::c_int != 0 && *s.offset(i as isize) as libc::c_int == '.' as i32 {
            *s.offset(i as isize) = '\0' as i32 as libc::c_char;
            first = 0 as libc::c_int != 0;
        }
        if *s.offset(i as isize) as libc::c_int == '/' as i32 {
            return &mut *s.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as *mut libc::c_char;
        }
        if i == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        i = i.wrapping_sub(1);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn scale_to_box(
    mut xmin: libc::c_double,
    mut ymin: libc::c_double,
    mut xmax: libc::c_double,
    mut ymax: libc::c_double,
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
) {
    let mut min: [libc::c_double; 3] = [0.; 3];
    let mut max: [libc::c_double; 3] = [0.; 3];
    let mut min0: [libc::c_double; 3] = [0.; 3];
    let mut ratio: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < dim {
        min[i as usize] = *x.offset(i as isize);
        max[i as usize] = *x.offset(i as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        k = 0 as libc::c_int;
        while k < dim {
            min[k as usize] = if *x.offset((i * dim + k) as isize) < min[k as usize] {
                *x.offset((i * dim + k) as isize)
            } else {
                min[k as usize]
            };
            max[k as usize] = if *x.offset((i * dim + k) as isize) > max[k as usize] {
                *x.offset((i * dim + k) as isize)
            } else {
                max[k as usize]
            };
            k += 1;
        }
        i += 1;
    }
    if max[0 as libc::c_int as usize] - min[0 as libc::c_int as usize]
        != 0 as libc::c_int as libc::c_double
    {
        ratio = (xmax - xmin) / (max[0 as libc::c_int as usize] - min[0 as libc::c_int as usize]);
    }
    if max[1 as libc::c_int as usize] - min[1 as libc::c_int as usize]
        != 0 as libc::c_int as libc::c_double
    {
        ratio = if ratio
            < (ymax - ymin) / (max[1 as libc::c_int as usize] - min[1 as libc::c_int as usize])
        {
            ratio
        } else {
            (ymax - ymin) / (max[1 as libc::c_int as usize] - min[1 as libc::c_int as usize])
        };
    }
    min0[0 as libc::c_int as usize] = xmin;
    min0[1 as libc::c_int as usize] = ymin;
    min0[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        k = 0 as libc::c_int;
        while k < dim {
            *x.offset((i * dim + k) as isize) =
                min0[k as usize] + (*x.offset((i * dim + k) as isize) - min[k as usize]) * ratio;
            k += 1;
        }
        i += 1;
    }
}
