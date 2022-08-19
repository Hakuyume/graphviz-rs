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
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn gdImageCreate(sx: libc::c_int, sy: libc::c_int) -> gdImagePtr;
    fn gdImageCreateFromPng(fd: *mut FILE) -> gdImagePtr;
    fn gdImageCreateFromGif(fd: *mut FILE) -> gdImagePtr;
    fn gdImageCreateFromJpeg(infile: *mut FILE) -> gdImagePtr;
    fn gdImageDestroy(im: gdImagePtr);
    fn gdImageSetPixel(im: gdImagePtr, x: libc::c_int, y: libc::c_int, color: libc::c_int);
    fn gdImageGetTrueColorPixel(im: gdImagePtr, x: libc::c_int, y: libc::c_int) -> libc::c_int;
    fn gdImageFilledRectangle(
        im: gdImagePtr,
        x1: libc::c_int,
        y1: libc::c_int,
        x2: libc::c_int,
        y2: libc::c_int,
        color: libc::c_int,
    );
    fn gdImageColorAllocate(
        im: gdImagePtr,
        r: libc::c_int,
        g: libc::c_int,
        b: libc::c_int,
    ) -> libc::c_int;
    fn gdImagePng(im: gdImagePtr, out: *mut FILE);
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type gdInterpolationMethod = libc::c_uint;
pub const GD_METHOD_COUNT: gdInterpolationMethod = 30;
pub const GD_WELSH: gdInterpolationMethod = 30;
pub const GD_COSINE: gdInterpolationMethod = 29;
pub const GD_CUBIC_SPLINE: gdInterpolationMethod = 28;
pub const GD_QUADRATIC_BSPLINE: gdInterpolationMethod = 27;
pub const GD_BLACKMAN_SINC: gdInterpolationMethod = 26;
pub const GD_BLACKMAN_BESSEL: gdInterpolationMethod = 25;
pub const GD_LANCZOS8: gdInterpolationMethod = 24;
pub const GD_LANCZOS3: gdInterpolationMethod = 23;
pub const GD_LINEAR: gdInterpolationMethod = 22;
pub const GD_WEIGHTED4: gdInterpolationMethod = 21;
pub const GD_TRIANGLE: gdInterpolationMethod = 20;
pub const GD_SINC: gdInterpolationMethod = 19;
pub const GD_QUADRATIC: gdInterpolationMethod = 18;
pub const GD_POWER: gdInterpolationMethod = 17;
pub const GD_NEAREST_NEIGHBOUR: gdInterpolationMethod = 16;
pub const GD_MITCHELL: gdInterpolationMethod = 15;
pub const GD_HANNING: gdInterpolationMethod = 14;
pub const GD_HAMMING: gdInterpolationMethod = 13;
pub const GD_HERMITE: gdInterpolationMethod = 12;
pub const GD_GENERALIZED_CUBIC: gdInterpolationMethod = 11;
pub const GD_GAUSSIAN: gdInterpolationMethod = 10;
pub const GD_CATMULLROM: gdInterpolationMethod = 9;
pub const GD_BSPLINE: gdInterpolationMethod = 8;
pub const GD_BOX: gdInterpolationMethod = 7;
pub const GD_BLACKMAN: gdInterpolationMethod = 6;
pub const GD_BICUBIC_FIXED: gdInterpolationMethod = 5;
pub const GD_BICUBIC: gdInterpolationMethod = 4;
pub const GD_BILINEAR_FIXED: gdInterpolationMethod = 3;
pub const GD_BESSEL: gdInterpolationMethod = 2;
pub const GD_BELL: gdInterpolationMethod = 1;
pub const GD_DEFAULT: gdInterpolationMethod = 0;
pub type interpolation_method =
    Option<unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdImageStruct {
    pub pixels: *mut *mut libc::c_uchar,
    pub sx: libc::c_int,
    pub sy: libc::c_int,
    pub colorsTotal: libc::c_int,
    pub red: [libc::c_int; 256],
    pub green: [libc::c_int; 256],
    pub blue: [libc::c_int; 256],
    pub open: [libc::c_int; 256],
    pub transparent: libc::c_int,
    pub polyInts: *mut libc::c_int,
    pub polyAllocated: libc::c_int,
    pub brush: *mut gdImageStruct,
    pub tile: *mut gdImageStruct,
    pub brushColorMap: [libc::c_int; 256],
    pub tileColorMap: [libc::c_int; 256],
    pub styleLength: libc::c_int,
    pub stylePos: libc::c_int,
    pub style: *mut libc::c_int,
    pub interlace: libc::c_int,
    pub thick: libc::c_int,
    pub alpha: [libc::c_int; 256],
    pub trueColor: libc::c_int,
    pub tpixels: *mut *mut libc::c_int,
    pub alphaBlendingFlag: libc::c_int,
    pub saveAlphaFlag: libc::c_int,
    pub AA: libc::c_int,
    pub AA_color: libc::c_int,
    pub AA_dont_blend: libc::c_int,
    pub cx1: libc::c_int,
    pub cy1: libc::c_int,
    pub cx2: libc::c_int,
    pub cy2: libc::c_int,
    pub res_x: libc::c_uint,
    pub res_y: libc::c_uint,
    pub paletteQuantizationMethod: libc::c_int,
    pub paletteQuantizationSpeed: libc::c_int,
    pub paletteQuantizationMinQuality: libc::c_int,
    pub paletteQuantizationMaxQuality: libc::c_int,
    pub interpolation_id: gdInterpolationMethod,
    pub interpolation: interpolation_method,
}
pub type gdImage = gdImageStruct;
pub type gdImagePtr = *mut gdImage;
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
unsafe extern "C" fn gv_alloc(mut size: size_t) -> *mut libc::c_void {
    return gv_calloc(1 as libc::c_int as size_t, size);
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
static mut pstopng: *mut libc::c_char = b"gs -dNOPAUSE -sDEVICE=pngalpha -sOutputFile=- -q -\0"
    as *const u8 as *const libc::c_char
    as *mut libc::c_char;
unsafe extern "C" fn imageLoad(mut filename: *mut libc::c_char) -> gdImagePtr {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut ext: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut im: gdImagePtr = 0 as *mut gdImage;
    let mut rc: libc::c_int = 0;
    let mut statbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    ext = strrchr(filename, '.' as i32);
    if ext.is_null() {
        fprintf(
            stderr,
            b"Filename \"%s\" has no file extension.\n\0" as *const u8 as *const libc::c_char,
            filename,
        );
        graphviz_exit(64 as libc::c_int);
    }
    rc = stat(filename, &mut statbuf);
    if rc != 0 {
        fprintf(
            stderr,
            b"Failed to stat \"%s\"\n\0" as *const u8 as *const libc::c_char,
            filename,
        );
        graphviz_exit(66 as libc::c_int);
    }
    if strcasecmp(ext, b".ps\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        ext = b".png\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        tmp = gv_alloc(
            (strlen(filename))
                .wrapping_add(strlen(ext))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(tmp, filename);
        strcat(tmp, ext);
        cmd = gv_alloc(
            (strlen(pstopng))
                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(filename))
                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(tmp))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(cmd, pstopng);
        strcat(cmd, b" <\0" as *const u8 as *const libc::c_char);
        strcat(cmd, filename);
        strcat(cmd, b" >\0" as *const u8 as *const libc::c_char);
        strcat(cmd, tmp);
        rc = system(cmd);
        free(cmd as *mut libc::c_void);
        f = fopen(tmp, b"rb\0" as *const u8 as *const libc::c_char);
        free(tmp as *mut libc::c_void);
        if f.is_null() {
            fprintf(
                stderr,
                b"Failed to open converted \"%s%s\"\n\0" as *const u8 as *const libc::c_char,
                filename,
                ext,
            );
            graphviz_exit(66 as libc::c_int);
        }
    } else {
        f = fopen(filename, b"rb\0" as *const u8 as *const libc::c_char);
        if f.is_null() {
            fprintf(
                stderr,
                b"Failed to open \"%s\"\n\0" as *const u8 as *const libc::c_char,
                filename,
            );
            graphviz_exit(66 as libc::c_int);
        }
    }
    im = 0 as gdImagePtr;
    if strcasecmp(ext, b".png\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        im = gdImageCreateFromPng(f);
    } else if strcasecmp(ext, b".gif\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        im = gdImageCreateFromGif(f);
    } else if strcasecmp(ext, b".jpg\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        im = gdImageCreateFromJpeg(f);
    }
    fclose(f);
    if im.is_null() {
        fprintf(
            stderr,
            b"Loading image from file  \"%s\" failed!\n\0" as *const u8 as *const libc::c_char,
            filename,
        );
        graphviz_exit(65 as libc::c_int);
    }
    return im;
}
unsafe extern "C" fn imageDiff(
    mut A: gdImagePtr,
    mut B: gdImagePtr,
    mut C: gdImagePtr,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut black: libc::c_int,
    mut white: libc::c_int,
) -> bool {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut d: bool = false;
    let mut rc: bool = false;
    rc = 0 as libc::c_int != 0;
    y = 0 as libc::c_int;
    while y < h {
        x = 0 as libc::c_int;
        while x < w {
            d = gdImageGetTrueColorPixel(B, x, y) - gdImageGetTrueColorPixel(A, x, y) != 0;
            gdImageSetPixel(C, x, y, if d as libc::c_int != 0 { white } else { black });
            rc = (rc as libc::c_int | d as libc::c_int) != 0;
            x += 1;
        }
        y += 1;
    }
    return rc;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut A: gdImagePtr = 0 as *mut gdImage;
    let mut B: gdImagePtr = 0 as *mut gdImage;
    let mut C: gdImagePtr = 0 as *mut gdImage;
    let mut black: libc::c_int = 0;
    let mut white: libc::c_int = 0;
    let mut minSX: libc::c_int = 0;
    let mut minSY: libc::c_int = 0;
    let mut maxSX: libc::c_int = 0;
    let mut maxSY: libc::c_int = 0;
    let mut rc: bool = false;
    let mut f: *mut FILE = 0 as *mut FILE;
    if argc == 2 as libc::c_int
        && strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"-?\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Usage: diffimg image1 image2 [outimage]\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(0 as libc::c_int);
    }
    if argc < 3 as libc::c_int {
        fprintf(
            stderr,
            b"Usage: diffimg image1 image2 [outimage]\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(64 as libc::c_int);
    }
    A = imageLoad(*argv.offset(1 as libc::c_int as isize));
    B = imageLoad(*argv.offset(2 as libc::c_int as isize));
    minSX = if (*A).sx < (*B).sx { (*A).sx } else { (*B).sx };
    minSY = if (*A).sy < (*B).sy { (*A).sy } else { (*B).sy };
    maxSX = if (*A).sx > (*B).sx { (*A).sx } else { (*B).sx };
    maxSY = if (*A).sy > (*B).sy { (*A).sy } else { (*B).sy };
    C = gdImageCreate(maxSX, maxSY);
    white = gdImageColorAllocate(
        C,
        255 as libc::c_int,
        255 as libc::c_int,
        255 as libc::c_int,
    );
    black = gdImageColorAllocate(C, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
    if maxSX > minSX && maxSY > minSY {
        gdImageFilledRectangle(
            C,
            minSX,
            minSY,
            maxSX - 1 as libc::c_int,
            maxSY - 1 as libc::c_int,
            black,
        );
    }
    rc = imageDiff(A, B, C, minSX, minSY, black, white);
    if argc > 3 as libc::c_int && {
        f = fopen(
            *argv.offset(3 as libc::c_int as isize),
            b"wb\0" as *const u8 as *const libc::c_char,
        );
        !f.is_null()
    } {
        gdImagePng(C, f);
        fclose(f);
    } else {
        gdImagePng(C, stdout);
    }
    gdImageDestroy(A);
    gdImageDestroy(B);
    gdImageDestroy(C);
    graphviz_exit(if rc as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    });
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
