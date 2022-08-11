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
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    static mut Verbose: libc::c_uchar;
    fn QuadTree_new_from_point_list(
        dim: libc::c_int,
        n: libc::c_int,
        max_level: libc::c_int,
        coord: *mut libc::c_double,
    ) -> QuadTree;
    fn color_palettes_get(color_palette_name: *mut libc::c_char) -> *mut libc::c_char;
    static lab_gamut_data: [libc::c_schar; 0];
    static mut lab_gamut_data_size: libc::c_int;
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
pub struct SingleLinkedList_struct {
    pub data: *mut libc::c_void,
    pub next: SingleLinkedList,
}
pub type SingleLinkedList = *mut SingleLinkedList_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QuadTree_struct {
    pub n: libc::c_int,
    pub total_weight: libc::c_double,
    pub dim: libc::c_int,
    pub center: *mut libc::c_double,
    pub width: libc::c_double,
    pub average: *mut libc::c_double,
    pub qts: *mut QuadTree,
    pub l: SingleLinkedList,
    pub max_level: libc::c_int,
    pub data: *mut libc::c_void,
}
pub type QuadTree = *mut QuadTree_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rgb_struct {
    pub r: libc::c_double,
    pub g: libc::c_double,
    pub b: libc::c_double,
}
pub type color_rgb = rgb_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xyz_struct {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub z: libc::c_double,
}
pub type color_xyz = xyz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lab_struct {
    pub l: libc::c_schar,
    pub a: libc::c_schar,
    pub b: libc::c_schar,
}
pub type color_lab = lab_struct;
#[no_mangle]
pub unsafe extern "C" fn color_rgb_init(
    mut r: libc::c_double,
    mut g: libc::c_double,
    mut b: libc::c_double,
) -> color_rgb {
    let mut rgb: color_rgb = color_rgb {
        r: 0.,
        g: 0.,
        b: 0.,
    };
    rgb.r = r;
    rgb.g = g;
    rgb.b = b;
    return rgb;
}
#[no_mangle]
pub unsafe extern "C" fn color_xyz_init(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
) -> color_xyz {
    let mut xyz: color_xyz = color_xyz {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    xyz.x = x;
    xyz.y = y;
    xyz.z = z;
    return xyz;
}
#[no_mangle]
pub unsafe extern "C" fn color_lab_init(
    mut l: libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> color_lab {
    let mut lab: color_lab = color_lab { l: 0, a: 0, b: 0 };
    lab.l = l as libc::c_schar;
    lab.a = a as libc::c_schar;
    lab.b = b as libc::c_schar;
    return lab;
}
#[no_mangle]
pub static mut XYZEpsilon: libc::c_double = 216.0f64 / 24389.0f64;
#[no_mangle]
pub static mut XYZKappa: libc::c_double = 24389.0f64 / 27.0f64;
unsafe extern "C" fn PivotXYZ(mut n: libc::c_double) -> libc::c_double {
    if n > XYZEpsilon {
        return pow(n, 1 as libc::c_int as libc::c_double / 3.0f64);
    }
    return (XYZKappa * n + 16 as libc::c_int as libc::c_double)
        / 116 as libc::c_int as libc::c_double;
}
unsafe extern "C" fn PivotRgb(mut n: libc::c_double) -> libc::c_double {
    if n > 0.04045f64 {
        return 100 as libc::c_int as libc::c_double * pow((n + 0.055f64) / 1.055f64, 2.4f64);
    }
    return 100 as libc::c_int as libc::c_double * n / 12.92f64;
}
#[no_mangle]
pub unsafe extern "C" fn RGB2XYZ(mut color: color_rgb) -> color_xyz {
    let mut r: libc::c_double = PivotRgb(color.r / 255.0f64);
    let mut g: libc::c_double = PivotRgb(color.g / 255.0f64);
    let mut b: libc::c_double = PivotRgb(color.b / 255.0f64);
    return color_xyz_init(
        r * 0.4124f64 + g * 0.3576f64 + b * 0.1805f64,
        r * 0.2126f64 + g * 0.7152f64 + b * 0.0722f64,
        r * 0.0193f64 + g * 0.1192f64 + b * 0.9505f64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn RGB2LAB(mut color: color_rgb) -> color_lab {
    let mut white: color_xyz = color_xyz_init(95.047f64, 100.000f64, 108.883f64);
    let mut xyz: color_xyz = RGB2XYZ(color);
    let mut x: libc::c_double = PivotXYZ(xyz.x / white.x);
    let mut y: libc::c_double = PivotXYZ(xyz.y / white.y);
    let mut z: libc::c_double = PivotXYZ(xyz.z / white.z);
    let mut L: libc::c_double = if 0 as libc::c_int as libc::c_double
        > 116 as libc::c_int as libc::c_double * y - 16 as libc::c_int as libc::c_double
    {
        0 as libc::c_int as libc::c_double
    } else {
        116 as libc::c_int as libc::c_double * y - 16 as libc::c_int as libc::c_double
    };
    let mut A: libc::c_double = 500 as libc::c_int as libc::c_double * (x - y);
    let mut B: libc::c_double = 200 as libc::c_int as libc::c_double * (y - z);
    return color_lab_init(L, A, B);
}
#[no_mangle]
pub unsafe extern "C" fn LAB2RGB_real_01(mut color: *mut libc::c_double) {
    let mut rgb: color_rgb = color_rgb {
        r: 0.,
        g: 0.,
        b: 0.,
    };
    let mut lab: color_lab = color_lab { l: 0, a: 0, b: 0 };
    lab.l = *color.offset(0 as libc::c_int as isize) as libc::c_schar;
    lab.a = *color.offset(1 as libc::c_int as isize) as libc::c_schar;
    lab.b = *color.offset(2 as libc::c_int as isize) as libc::c_schar;
    rgb = LAB2RGB(lab);
    *color.offset(0 as libc::c_int as isize) = rgb.r / 255 as libc::c_int as libc::c_double;
    *color.offset(1 as libc::c_int as isize) = rgb.g / 255 as libc::c_int as libc::c_double;
    *color.offset(2 as libc::c_int as isize) = rgb.b / 255 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn LAB2RGB(mut color: color_lab) -> color_rgb {
    let mut y: libc::c_double = (color.l as libc::c_int as libc::c_double + 16.0f64) / 116.0f64;
    let mut x: libc::c_double = color.a as libc::c_int as libc::c_double / 500.0f64 + y;
    let mut z: libc::c_double = y - color.b as libc::c_int as libc::c_double / 200.0f64;
    let mut white: color_xyz = color_xyz_init(95.047f64, 100.000f64, 108.883f64);
    let mut xyz: color_xyz = color_xyz {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut t1: libc::c_double = 0.;
    let mut t2: libc::c_double = 0.;
    let mut t3: libc::c_double = 0.;
    if pow(x, 3.0f64) > XYZEpsilon {
        t1 = pow(x, 3.0f64);
    } else {
        t1 = (x - 16.0f64 / 116.0f64) / 7.787f64;
    }
    if color.l as libc::c_int as libc::c_double > XYZKappa * XYZEpsilon {
        t2 = pow(
            (color.l as libc::c_int as libc::c_double + 16.0f64) / 116.0f64,
            3.0f64,
        );
    } else {
        t2 = color.l as libc::c_int as libc::c_double / XYZKappa;
    }
    if pow(z, 3.0f64) > XYZEpsilon {
        t3 = pow(z, 3.0f64);
    } else {
        t3 = (z - 16.0f64 / 116.0f64) / 7.787f64;
    }
    xyz = color_xyz_init(white.x * t1, white.y * t2, white.z * t3);
    return XYZ2RGB(xyz);
}
#[no_mangle]
pub unsafe extern "C" fn XYZ2RGB(mut color: color_xyz) -> color_rgb {
    let mut x: libc::c_double = color.x / 100.0f64;
    let mut y: libc::c_double = color.y / 100.0f64;
    let mut z: libc::c_double = color.z / 100.0f64;
    let mut r: libc::c_double = x * 3.2406f64 + y * -1.5372f64 + z * -0.4986f64;
    let mut g: libc::c_double = x * -0.9689f64 + y * 1.8758f64 + z * 0.0415f64;
    let mut b: libc::c_double = x * 0.0557f64 + y * -0.2040f64 + z * 1.0570f64;
    if r > 0.0031308f64 {
        r = 1.055f64 * pow(r, 1 as libc::c_int as libc::c_double / 2.4f64) - 0.055f64;
    } else {
        r = 12.92f64 * r;
    }
    if g > 0.0031308f64 {
        g = 1.055f64 * pow(g, 1 as libc::c_int as libc::c_double / 2.4f64) - 0.055f64;
    } else {
        g = 12.92f64 * g;
    }
    if b > 0.0031308f64 {
        b = 1.055f64 * pow(b, 1 as libc::c_int as libc::c_double / 2.4f64) - 0.055f64;
    } else {
        b = 12.92f64 * b;
    }
    r = if 0 as libc::c_int as libc::c_double > r {
        0 as libc::c_int as libc::c_double
    } else {
        r
    };
    r = if (255 as libc::c_int as libc::c_double) < r * 255 as libc::c_int as libc::c_double {
        255 as libc::c_int as libc::c_double
    } else {
        r * 255 as libc::c_int as libc::c_double
    };
    g = if 0 as libc::c_int as libc::c_double > g {
        0 as libc::c_int as libc::c_double
    } else {
        g
    };
    g = if (255 as libc::c_int as libc::c_double) < g * 255 as libc::c_int as libc::c_double {
        255 as libc::c_int as libc::c_double
    } else {
        g * 255 as libc::c_int as libc::c_double
    };
    b = if 0 as libc::c_int as libc::c_double > b {
        0 as libc::c_int as libc::c_double
    } else {
        b
    };
    b = if (255 as libc::c_int as libc::c_double) < b * 255 as libc::c_int as libc::c_double {
        255 as libc::c_int as libc::c_double
    } else {
        b * 255 as libc::c_int as libc::c_double
    };
    return color_rgb_init(r, g, b);
}
#[no_mangle]
pub unsafe extern "C" fn lab_gamut(
    mut lightness: *const libc::c_char,
    mut n: *mut libc::c_int,
) -> *mut libc::c_double {
    let mut xx: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut l1: libc::c_int = 0 as libc::c_int;
    let mut l2: libc::c_int = 70 as libc::c_int;
    if !lightness.is_null()
        && sscanf(
            lightness,
            b"%d,%d\0" as *const u8 as *const libc::c_char,
            &mut l1 as *mut libc::c_int,
            &mut l2 as *mut libc::c_int,
        ) == 2 as libc::c_int
    {
        if l1 < 0 as libc::c_int {
            l1 = 0 as libc::c_int;
        }
        if l2 > 100 as libc::c_int {
            l2 = 100 as libc::c_int;
        }
        if l1 > l2 {
            l1 = l2;
        }
    } else {
        l1 = 0 as libc::c_int;
        l2 = 70 as libc::c_int;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"LAB color lightness range = %d,%d\n\0" as *const u8 as *const libc::c_char,
            l1,
            l2,
        );
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"size of lab gamut = %d\n\0" as *const u8 as *const libc::c_char,
            lab_gamut_data_size,
        );
    }
    let mut m: size_t = (l2 as size_t)
        .wrapping_sub(l1 as size_t)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(256 as libc::c_int as libc::c_ulong)
        .wrapping_mul(256 as libc::c_int as libc::c_ulong)
        .wrapping_mul(3 as libc::c_int as libc::c_ulong);
    x = malloc((::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul(m))
        as *mut libc::c_double;
    xx = x;
    *n = 0 as libc::c_int;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < lab_gamut_data_size as size_t {
        if *lab_gamut_data.as_ptr().offset(i as isize) as libc::c_int >= l1
            && *lab_gamut_data.as_ptr().offset(i as isize) as libc::c_int <= l2
        {
            let mut b_lower: libc::c_int = *lab_gamut_data
                .as_ptr()
                .offset(i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int;
            let mut b_upper: libc::c_int = *lab_gamut_data
                .as_ptr()
                .offset(i.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int;
            let mut b: libc::c_int = b_lower;
            while b <= b_upper {
                *xx.offset(0 as libc::c_int as isize) =
                    *lab_gamut_data.as_ptr().offset(i as isize) as libc::c_double;
                *xx.offset(1 as libc::c_int as isize) = *lab_gamut_data
                    .as_ptr()
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_double;
                *xx.offset(2 as libc::c_int as isize) = b as libc::c_double;
                xx = xx.offset(3 as libc::c_int as isize);
                *n += 1;
                b += 1;
            }
        }
        i = (i as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
            as size_t;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn lab_gamut_quadtree(
    mut lightness: *const libc::c_char,
    mut max_qtree_level: libc::c_int,
) -> QuadTree {
    let mut n: libc::c_int = 0;
    let mut x: *mut libc::c_double = lab_gamut(lightness, &mut n);
    let mut qt: QuadTree = 0 as *mut QuadTree_struct;
    let mut dim: libc::c_int = 3 as libc::c_int;
    if x.is_null() {
        return 0 as QuadTree;
    }
    qt = QuadTree_new_from_point_list(dim, n, max_qtree_level, x);
    free(x as *mut libc::c_void);
    return qt;
}
unsafe extern "C" fn lab_dist(mut x: color_lab, mut y: color_lab) -> libc::c_double {
    return sqrt(
        ((x.l as libc::c_int - y.l as libc::c_int) * (x.l as libc::c_int - y.l as libc::c_int)
            + (x.a as libc::c_int - y.a as libc::c_int) * (x.a as libc::c_int - y.a as libc::c_int)
            + (x.b as libc::c_int - y.b as libc::c_int) * (x.b as libc::c_int - y.b as libc::c_int))
            as libc::c_double,
    );
}
unsafe extern "C" fn lab_interpolate(
    mut lab1: color_lab,
    mut lab2: color_lab,
    mut t: libc::c_double,
    mut colors: *mut libc::c_double,
) {
    *colors.offset(0 as libc::c_int as isize) = lab1.l as libc::c_int as libc::c_double
        + t * (lab2.l as libc::c_int - lab1.l as libc::c_int) as libc::c_double;
    *colors.offset(1 as libc::c_int as isize) = lab1.a as libc::c_int as libc::c_double
        + t * (lab2.a as libc::c_int - lab1.a as libc::c_int) as libc::c_double;
    *colors.offset(2 as libc::c_int as isize) = lab1.b as libc::c_int as libc::c_double
        + t * (lab2.b as libc::c_int - lab1.b as libc::c_int) as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn color_blend_rgb2lab(
    mut color_list: *mut libc::c_char,
    maxpoints: libc::c_int,
    mut colors0: *mut *mut libc::c_double,
) {
    let mut nc: libc::c_int = 1 as libc::c_int;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut cdim: libc::c_int = 3 as libc::c_int;
    let mut cl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lab: *mut color_lab = 0 as *mut color_lab;
    let mut rgb: color_rgb = color_rgb {
        r: 0.,
        g: 0.,
        b: 0.,
    };
    let mut dists: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut step: libc::c_double = 0.;
    let mut dist_current: libc::c_double = 0.;
    let mut colors: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = color_palettes_get(color_list);
    if !cp.is_null() {
        color_list = cp;
    }
    if maxpoints <= 0 as libc::c_int {
        return;
    }
    cl = color_list;
    loop {
        cl = strchr(cl, ',' as i32);
        if cl.is_null() {
            break;
        }
        cl = cl.offset(1);
        nc += 1;
    }
    lab = malloc(
        (::std::mem::size_of::<color_lab>() as libc::c_ulong).wrapping_mul(
            (if nc > 1 as libc::c_int {
                nc
            } else {
                1 as libc::c_int
            }) as libc::c_ulong,
        ),
    ) as *mut color_lab;
    cl = color_list.offset(-(1 as libc::c_int as isize));
    nc = 0 as libc::c_int;
    loop {
        cl = cl.offset(1);
        if sscanf(
            cl,
            b"#%02X%02X%02X\0" as *const u8 as *const libc::c_char,
            &mut r as *mut libc::c_int,
            &mut g as *mut libc::c_int,
            &mut b as *mut libc::c_int,
        ) != 3 as libc::c_int
        {
            break;
        }
        rgb.r = r as libc::c_double;
        rgb.g = g as libc::c_double;
        rgb.b = b as libc::c_double;
        let fresh0 = nc;
        nc = nc + 1;
        *lab.offset(fresh0 as isize) = RGB2LAB(rgb);
        cl = strchr(cl, ',' as i32);
        if cl.is_null() {
            break;
        }
    }
    dists = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul(
            (if 1 as libc::c_int > nc {
                1 as libc::c_int
            } else {
                nc
            }) as libc::c_ulong,
        ),
    ) as *mut libc::c_double;
    *dists.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < nc - 1 as libc::c_int {
        *dists.offset((i + 1 as libc::c_int) as isize) = lab_dist(
            *lab.offset(i as isize),
            *lab.offset((i + 1 as libc::c_int) as isize),
        );
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < nc - 1 as libc::c_int {
        *dists.offset((i + 1 as libc::c_int) as isize) += *dists.offset(i as isize);
        i += 1;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"sum = %f\n\0" as *const u8 as *const libc::c_char,
            *dists.offset((nc - 1 as libc::c_int) as isize),
        );
    }
    if (*colors0).is_null() {
        *colors0 = malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(maxpoints as libc::c_ulong)
                .wrapping_mul(cdim as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    colors = *colors0;
    if maxpoints == 1 as libc::c_int {
        *colors.offset(0 as libc::c_int as isize) =
            (*lab.offset(0 as libc::c_int as isize)).l as libc::c_double;
        *colors.offset(1 as libc::c_int as isize) =
            (*lab.offset(0 as libc::c_int as isize)).a as libc::c_double;
        *colors.offset(2 as libc::c_int as isize) =
            (*lab.offset(0 as libc::c_int as isize)).b as libc::c_double;
    } else {
        step = *dists.offset((nc - 1 as libc::c_int) as isize)
            / (maxpoints - 1 as libc::c_int) as libc::c_double;
        ii = 0 as libc::c_int;
        jj = 0 as libc::c_int;
        dist_current = 0 as libc::c_int as libc::c_double;
        while *dists.offset(jj as isize) < *dists.offset(ii as isize) + step {
            jj += 1;
        }
        i = 0 as libc::c_int;
        while i < maxpoints {
            lab_interpolate(
                *lab.offset(ii as isize),
                *lab.offset(jj as isize),
                (dist_current - *dists.offset(ii as isize))
                    / (if 0.001f64 > *dists.offset(jj as isize) - *dists.offset(ii as isize) {
                        0.001f64
                    } else {
                        *dists.offset(jj as isize) - *dists.offset(ii as isize)
                    }),
                colors,
            );
            dist_current += step;
            colors = colors.offset(cdim as isize);
            if dist_current > *dists.offset(jj as isize) {
                ii = jj;
            }
            while jj < nc - 1 as libc::c_int
                && *dists.offset(jj as isize) < *dists.offset(ii as isize) + step
            {
                jj += 1;
            }
            i += 1;
        }
    }
    free(dists as *mut libc::c_void);
    free(lab as *mut libc::c_void);
}
