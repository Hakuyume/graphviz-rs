#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn free(_: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn gdFTUseFontConfig(flag: libc::c_int) -> libc::c_int;
    fn gdImageStringFTEx(
        im: gdImagePtr,
        brect: *mut libc::c_int,
        fg: libc::c_int,
        fontlist: *const libc::c_char,
        ptsize: libc::c_double,
        angle: libc::c_double,
        x: libc::c_int,
        y: libc::c_int,
        string: *const libc::c_char,
        strex: gdFTStringExtraPtr,
    ) -> *mut libc::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pointf_s {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type pointf = pointf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PostscriptAlias {
    pub name: *mut libc::c_char,
    pub family: *mut libc::c_char,
    pub weight: *mut libc::c_char,
    pub stretch: *mut libc::c_char,
    pub style: *mut libc::c_char,
    pub xfig_code: libc::c_int,
    pub svg_font_family: *mut libc::c_char,
    pub svg_font_weight: *mut libc::c_char,
    pub svg_font_style: *mut libc::c_char,
}
pub type PostscriptAlias = _PostscriptAlias;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct textfont_t {
    pub name: *mut libc::c_char,
    pub color: *mut libc::c_char,
    pub postscript_alias: *mut PostscriptAlias,
    pub size: libc::c_double,
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "0..=6")]
    #[bitfield(name = "cnt", ty = "libc::c_uint", bits = "7..=31")]
    pub flags_cnt: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textspan_t {
    pub str_0: *mut libc::c_char,
    pub font: *mut textfont_t,
    pub layout: *mut libc::c_void,
    pub free_layout: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub yoffset_layout: libc::c_double,
    pub yoffset_centerline: libc::c_double,
    pub size: pointf,
    pub just: libc::c_char,
}
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
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
pub struct gvtextlayout_engine_s {
    pub textlayout: Option::<
        unsafe extern "C" fn(*mut textspan_t, *mut *mut libc::c_char) -> bool,
    >,
}
pub type gvtextlayout_engine_t = gvtextlayout_engine_s;
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
pub type interpolation_method = Option::<
    unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdFTStringExtra {
    pub flags: libc::c_int,
    pub linespacing: libc::c_double,
    pub charmap: libc::c_int,
    pub hdpi: libc::c_int,
    pub vdpi: libc::c_int,
    pub xshow: *mut libc::c_char,
    pub fontpath: *mut libc::c_char,
}
pub type gdFTStringExtraPtr = *mut gdFTStringExtra;
#[no_mangle]
pub unsafe extern "C" fn gd_psfontResolve(
    mut pa: *mut PostscriptAlias,
) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut comma: libc::c_int = 0 as libc::c_int;
    strcpy(buf.as_mut_ptr(), (*pa).family);
    if !((*pa).weight).is_null() {
        strcat(
            buf.as_mut_ptr(),
            if comma != 0 {
                b" \0" as *const u8 as *const libc::c_char
            } else {
                b", \0" as *const u8 as *const libc::c_char
            },
        );
        comma = 1 as libc::c_int;
        strcat(buf.as_mut_ptr(), (*pa).weight);
    }
    if !((*pa).stretch).is_null() {
        strcat(
            buf.as_mut_ptr(),
            if comma != 0 {
                b" \0" as *const u8 as *const libc::c_char
            } else {
                b", \0" as *const u8 as *const libc::c_char
            },
        );
        comma = 1 as libc::c_int;
        strcat(buf.as_mut_ptr(), (*pa).stretch);
    }
    if !((*pa).style).is_null() {
        strcat(
            buf.as_mut_ptr(),
            if comma != 0 {
                b" \0" as *const u8 as *const libc::c_char
            } else {
                b", \0" as *const u8 as *const libc::c_char
            },
        );
        comma = 1 as libc::c_int;
        strcat(buf.as_mut_ptr(), (*pa).style);
    }
    return buf.as_mut_ptr();
}
unsafe extern "C" fn gd_textlayout(
    mut span: *mut textspan_t,
    mut fontpath: *mut *mut libc::c_char,
) -> bool {
    let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fontlist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fontname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fontsize: libc::c_double = 0.;
    let mut brect: [libc::c_int; 8] = [0; 8];
    let mut strex: gdFTStringExtra = gdFTStringExtra {
        flags: 0,
        linespacing: 0.,
        charmap: 0,
        hdpi: 0,
        vdpi: 0,
        xshow: 0 as *mut libc::c_char,
        fontpath: 0 as *mut libc::c_char,
    };
    let mut pA: *mut PostscriptAlias = 0 as *mut PostscriptAlias;
    fontname = (*(*span).font).name;
    fontsize = (*(*span).font).size;
    strex.fontpath = 0 as *mut libc::c_char;
    strex.flags = 128 as libc::c_int | 4 as libc::c_int;
    strex.vdpi = 72 as libc::c_int;
    strex.hdpi = strex.vdpi;
    if !(strchr(fontname, '/' as i32)).is_null() {
        strex.flags |= 32 as libc::c_int;
    } else {
        strex.flags |= 64 as libc::c_int;
    }
    (*span).size.x = 0.0f64;
    (*span).size.y = 0.0f64;
    (*span).yoffset_layout = 0.0f64;
    let ref mut fresh0 = (*span).layout;
    *fresh0 = 0 as *mut libc::c_void;
    let ref mut fresh1 = (*span).free_layout;
    *fresh1 = None;
    (*span).yoffset_centerline = 0.1f64 * fontsize;
    if !fontname.is_null() {
        if fontsize <= 0.15f64 {
            return 1 as libc::c_int != 0
        } else {
            if fontsize <= 1.5f64 {
                fontsize = 1.5f64;
            }
        }
        gdFTUseFontConfig(1 as libc::c_int);
        pA = (*(*span).font).postscript_alias;
        if !pA.is_null() {
            fontlist = gd_psfontResolve(pA);
        } else {
            fontlist = fontname;
        }
        err = gdImageStringFTEx(
            0 as gdImagePtr,
            brect.as_mut_ptr(),
            -(1 as libc::c_int),
            fontlist,
            fontsize,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int,
            0 as libc::c_int,
            (*span).str_0,
            &mut strex,
        );
        if !err.is_null() {
            agerr(AGERR, b"%s\n\0" as *const u8 as *const libc::c_char, err);
            return 0 as libc::c_int != 0;
        }
        if !fontpath.is_null() {
            *fontpath = strex.fontpath;
        } else {
            free(strex.fontpath as *mut libc::c_void);
        }
        if !((*span).str_0).is_null()
            && *((*span).str_0).offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            (*span)
                .size
                .x = (brect[4 as libc::c_int as usize]
                - brect[0 as libc::c_int as usize]) as libc::c_double;
            (*span).size.y = (fontsize * 1.2f64) as libc::c_int as libc::c_double;
        }
    }
    return 1 as libc::c_int != 0;
}
static mut gd_textlayout_engine: gvtextlayout_engine_t = unsafe {
    {
        let mut init = gvtextlayout_engine_s {
            textlayout: Some(
                gd_textlayout
                    as unsafe extern "C" fn(
                        *mut textspan_t,
                        *mut *mut libc::c_char,
                    ) -> bool,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gvtextlayout_gd_types: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: 0 as libc::c_int,
                type_0: b"textlayout\0" as *const u8 as *const libc::c_char,
                quality: 2 as libc::c_int,
                engine: &gd_textlayout_engine as *const gvtextlayout_engine_t
                    as *mut gvtextlayout_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
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
