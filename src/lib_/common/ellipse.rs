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
#![feature(register_tool)]
extern "C" {
    fn atan(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pointf_s {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type pointf = pointf_s;
pub type Ppoint_t = pointf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ppoly_t {
    pub ps: *mut Ppoint_t,
    pub pn: libc::c_int,
}
pub type Ppolyline_t = Ppoly_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ellipse_t {
    pub cx: libc::c_double,
    pub cy: libc::c_double,
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub theta: libc::c_double,
    pub cosTheta: libc::c_double,
    pub sinTheta: libc::c_double,
    pub eta1: libc::c_double,
    pub eta2: libc::c_double,
    pub x1: libc::c_double,
    pub y1: libc::c_double,
    pub x2: libc::c_double,
    pub y2: libc::c_double,
    pub xF1: libc::c_double,
    pub yF1: libc::c_double,
    pub xF2: libc::c_double,
    pub yF2: libc::c_double,
    pub xLeft: libc::c_double,
    pub yUp: libc::c_double,
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub f: libc::c_double,
    pub e2: libc::c_double,
    pub g: libc::c_double,
    pub g2: libc::c_double,
}
pub type erray_t = [[[libc::c_double; 4]; 4]; 2];
unsafe extern "C" fn computeFoci(mut ep: *mut ellipse_t) {
    let mut d: libc::c_double = sqrt((*ep).a * (*ep).a - (*ep).b * (*ep).b);
    let mut dx: libc::c_double = d * (*ep).cosTheta;
    let mut dy: libc::c_double = d * (*ep).sinTheta;
    (*ep).xF1 = (*ep).cx - dx;
    (*ep).yF1 = (*ep).cy - dy;
    (*ep).xF2 = (*ep).cx + dx;
    (*ep).yF2 = (*ep).cy + dy;
}
unsafe extern "C" fn computeEndPoints(mut ep: *mut ellipse_t) {
    let mut aCosEta1: libc::c_double = (*ep).a * cos((*ep).eta1);
    let mut bSinEta1: libc::c_double = (*ep).b * sin((*ep).eta1);
    let mut aCosEta2: libc::c_double = (*ep).a * cos((*ep).eta2);
    let mut bSinEta2: libc::c_double = (*ep).b * sin((*ep).eta2);
    (*ep).x1 = (*ep).cx + aCosEta1 * (*ep).cosTheta - bSinEta1 * (*ep).sinTheta;
    (*ep).y1 = (*ep).cy + aCosEta1 * (*ep).sinTheta + bSinEta1 * (*ep).cosTheta;
    (*ep).x2 = (*ep).cx + aCosEta2 * (*ep).cosTheta - bSinEta2 * (*ep).sinTheta;
    (*ep).y2 = (*ep).cy + aCosEta2 * (*ep).sinTheta + bSinEta2 * (*ep).cosTheta;
}
unsafe extern "C" fn computeBounds(mut ep: *mut ellipse_t) {
    let mut bOnA: libc::c_double = (*ep).b / (*ep).a;
    let mut etaXMin: libc::c_double = 0.;
    let mut etaXMax: libc::c_double = 0.;
    let mut etaYMin: libc::c_double = 0.;
    let mut etaYMax: libc::c_double = 0.;
    if fabs((*ep).sinTheta) < 0.1f64 {
        let mut tanTheta: libc::c_double = (*ep).sinTheta / (*ep).cosTheta;
        if (*ep).cosTheta < 0 as libc::c_int as libc::c_double {
            etaXMin = -atan(tanTheta * bOnA);
            etaXMax = etaXMin + 3.14159265358979323846f64;
            etaYMin = 0.5f64 * 3.14159265358979323846f64 - atan(tanTheta / bOnA);
            etaYMax = etaYMin + 3.14159265358979323846f64;
        } else {
            etaXMax = -atan(tanTheta * bOnA);
            etaXMin = etaXMax - 3.14159265358979323846f64;
            etaYMax = 0.5f64 * 3.14159265358979323846f64 - atan(tanTheta / bOnA);
            etaYMin = etaYMax - 3.14159265358979323846f64;
        }
    } else {
        let mut invTanTheta: libc::c_double = (*ep).cosTheta / (*ep).sinTheta;
        if (*ep).sinTheta < 0 as libc::c_int as libc::c_double {
            etaXMax = 0.5f64 * 3.14159265358979323846f64 + atan(invTanTheta / bOnA);
            etaXMin = etaXMax - 3.14159265358979323846f64;
            etaYMin = atan(invTanTheta * bOnA);
            etaYMax = etaYMin + 3.14159265358979323846f64;
        } else {
            etaXMin = 0.5f64 * 3.14159265358979323846f64 + atan(invTanTheta / bOnA);
            etaXMax = etaXMin + 3.14159265358979323846f64;
            etaYMax = atan(invTanTheta * bOnA);
            etaYMin = etaYMax - 3.14159265358979323846f64;
        }
    }
    etaXMin -= 2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        * floor(
            (etaXMin - (*ep).eta1)
                / (2 as libc::c_int as libc::c_double * 3.14159265358979323846f64),
        );
    etaYMin -= 2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        * floor(
            (etaYMin - (*ep).eta1)
                / (2 as libc::c_int as libc::c_double * 3.14159265358979323846f64),
        );
    etaXMax -= 2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        * floor(
            (etaXMax - (*ep).eta1)
                / (2 as libc::c_int as libc::c_double * 3.14159265358979323846f64),
        );
    etaYMax -= 2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        * floor(
            (etaYMax - (*ep).eta1)
                / (2 as libc::c_int as libc::c_double * 3.14159265358979323846f64),
        );
    (*ep).xLeft = if etaXMin <= (*ep).eta2 {
        (*ep).cx + (*ep).a * cos(etaXMin) * (*ep).cosTheta - (*ep).b * sin(etaXMin) * (*ep).sinTheta
    } else if (*ep).x1 < (*ep).x2 {
        (*ep).x1
    } else {
        (*ep).x2
    };
    (*ep).yUp = if etaYMin <= (*ep).eta2 {
        (*ep).cy + (*ep).a * cos(etaYMin) * (*ep).sinTheta + (*ep).b * sin(etaYMin) * (*ep).cosTheta
    } else if (*ep).y1 < (*ep).y2 {
        (*ep).y1
    } else {
        (*ep).y2
    };
    (*ep).width = (if etaXMax <= (*ep).eta2 {
        (*ep).cx + (*ep).a * cos(etaXMax) * (*ep).cosTheta - (*ep).b * sin(etaXMax) * (*ep).sinTheta
    } else {
        (if (*ep).x1 > (*ep).x2 {
            (*ep).x1
        } else {
            (*ep).x2
        })
    }) - (*ep).xLeft;
    (*ep).height = (if etaYMax <= (*ep).eta2 {
        (*ep).cy + (*ep).a * cos(etaYMax) * (*ep).sinTheta + (*ep).b * sin(etaYMax) * (*ep).cosTheta
    } else {
        (if (*ep).y1 > (*ep).y2 {
            (*ep).y1
        } else {
            (*ep).y2
        })
    }) - (*ep).yUp;
}
unsafe extern "C" fn initEllipse(
    mut ep: *mut ellipse_t,
    mut cx: libc::c_double,
    mut cy: libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut theta: libc::c_double,
    mut lambda1: libc::c_double,
    mut lambda2: libc::c_double,
) {
    (*ep).cx = cx;
    (*ep).cy = cy;
    (*ep).a = a;
    (*ep).b = b;
    (*ep).theta = theta;
    (*ep).eta1 = atan2(sin(lambda1) / b, cos(lambda1) / a);
    (*ep).eta2 = atan2(sin(lambda2) / b, cos(lambda2) / a);
    (*ep).cosTheta = cos(theta);
    (*ep).sinTheta = sin(theta);
    (*ep).eta2 -= 2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        * floor(
            ((*ep).eta2 - (*ep).eta1)
                / (2 as libc::c_int as libc::c_double * 3.14159265358979323846f64),
        );
    if lambda2 - lambda1 > 3.14159265358979323846f64
        && (*ep).eta2 - (*ep).eta1 < 3.14159265358979323846f64
    {
        (*ep).eta2 += 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64;
    }
    computeFoci(ep);
    computeEndPoints(ep);
    computeBounds(ep);
    (*ep).f = ((*ep).a - (*ep).b) / (*ep).a;
    (*ep).e2 = (*ep).f * (2.0f64 - (*ep).f);
    (*ep).g = 1.0f64 - (*ep).f;
    (*ep).g2 = (*ep).g * (*ep).g;
}
static mut coeffs2Low: erray_t = [
    [
        [3.92478f64, -13.5822f64, -0.233377f64, 0.0128206f64],
        [-1.08814f64, 0.859987f64, 0.000362265f64, 0.000229036f64],
        [-0.942512f64, 0.390456f64, 0.0080909f64, 0.00723895f64],
        [-0.736228f64, 0.20998f64, 0.0129867f64, 0.0103456f64],
    ],
    [
        [-0.395018f64, 6.82464f64, 0.0995293f64, 0.0122198f64],
        [-0.545608f64, 0.0774863f64, 0.0267327f64, 0.0132482f64],
        [0.0534754f64, -0.0884167f64, 0.012595f64, 0.0343396f64],
        [0.209052f64, -0.0599987f64, -0.00723897f64, 0.00789976f64],
    ],
];
static mut coeffs2High: erray_t = [
    [
        [0.0863805f64, -11.5595f64, -2.68765f64, 0.181224f64],
        [0.242856f64, -1.81073f64, 1.56876f64, 1.68544f64],
        [0.233337f64, -0.455621f64, 0.222856f64, 0.403469f64],
        [0.0612978f64, -0.104879f64, 0.0446799f64, 0.00867312f64],
    ],
    [
        [0.028973f64, 6.68407f64, 0.171472f64, 0.0211706f64],
        [0.0307674f64, -0.0517815f64, 0.0216803f64, -0.0749348f64],
        [-0.0471179f64, 0.1288f64, -0.0781702f64, 2.0f64],
        [-0.0309683f64, 0.0531557f64, -0.0227191f64, 0.0434511f64],
    ],
];
static mut safety2: [libc::c_double; 4] = [0.02f64, 2.83f64, 0.125f64, 0.01f64];
static mut coeffs3Low: erray_t = [
    [
        [3.85268f64, -21.229f64, -0.330434f64, 0.0127842f64],
        [-1.61486f64, 0.706564f64, 0.225945f64, 0.263682f64],
        [-0.910164f64, 0.388383f64, 0.00551445f64, 0.00671814f64],
        [-0.630184f64, 0.192402f64, 0.0098871f64, 0.0102527f64],
    ],
    [
        [-0.162211f64, 9.94329f64, 0.13723f64, 0.0124084f64],
        [-0.253135f64, 0.00187735f64, 0.0230286f64, 0.01264f64],
        [-0.0695069f64, -0.0437594f64, 0.0120636f64, 0.0163087f64],
        [-0.0328856f64, -0.00926032f64, -0.00173573f64, 0.00527385f64],
    ],
];
static mut coeffs3High: erray_t = [
    [
        [0.0899116f64, -19.2349f64, -4.11711f64, 0.183362f64],
        [0.138148f64, -1.45804f64, 1.32044f64, 1.38474f64],
        [0.230903f64, -0.450262f64, 0.219963f64, 0.414038f64],
        [0.0590565f64, -0.101062f64, 0.0430592f64, 0.0204699f64],
    ],
    [
        [0.0164649f64, 9.89394f64, 0.0919496f64, 0.00760802f64],
        [0.0191603f64, -0.0322058f64, 0.0134667f64, -0.0825018f64],
        [0.0156192f64, -0.017535f64, 0.00326508f64, -0.228157f64],
        [-0.0236752f64, 0.0405821f64, -0.0173086f64, 0.176187f64],
    ],
];
static mut safety3: [libc::c_double; 4] = [0.001f64, 4.98f64, 0.207f64, 0.0067f64];
unsafe extern "C" fn estimateError(
    mut ep: *mut ellipse_t,
    mut degree: libc::c_int,
    mut etaA: libc::c_double,
    mut etaB: libc::c_double,
) -> libc::c_double {
    let mut c0: libc::c_double = 0.;
    let mut c1: libc::c_double = 0.;
    let mut eta: libc::c_double = 0.5f64 * (etaA + etaB);
    if degree < 2 as libc::c_int {
        let mut aCosEtaA: libc::c_double = (*ep).a * cos(etaA);
        let mut bSinEtaA: libc::c_double = (*ep).b * sin(etaA);
        let mut xA: libc::c_double =
            (*ep).cx + aCosEtaA * (*ep).cosTheta - bSinEtaA * (*ep).sinTheta;
        let mut yA: libc::c_double =
            (*ep).cy + aCosEtaA * (*ep).sinTheta + bSinEtaA * (*ep).cosTheta;
        let mut aCosEtaB: libc::c_double = (*ep).a * cos(etaB);
        let mut bSinEtaB: libc::c_double = (*ep).b * sin(etaB);
        let mut xB: libc::c_double =
            (*ep).cx + aCosEtaB * (*ep).cosTheta - bSinEtaB * (*ep).sinTheta;
        let mut yB: libc::c_double =
            (*ep).cy + aCosEtaB * (*ep).sinTheta + bSinEtaB * (*ep).cosTheta;
        let mut aCosEta: libc::c_double = (*ep).a * cos(eta);
        let mut bSinEta: libc::c_double = (*ep).b * sin(eta);
        let mut x: libc::c_double = (*ep).cx + aCosEta * (*ep).cosTheta - bSinEta * (*ep).sinTheta;
        let mut y: libc::c_double = (*ep).cy + aCosEta * (*ep).sinTheta + bSinEta * (*ep).cosTheta;
        let mut dx: libc::c_double = xB - xA;
        let mut dy: libc::c_double = yB - yA;
        return fabs(x * dy - y * dx + xB * yA - xA * yB) / hypot(dx, dy);
    } else {
        let mut x_0: libc::c_double = (*ep).b / (*ep).a;
        let mut dEta: libc::c_double = etaB - etaA;
        let mut cos2: libc::c_double = cos(2 as libc::c_int as libc::c_double * eta);
        let mut cos4: libc::c_double = cos(4 as libc::c_int as libc::c_double * eta);
        let mut cos6: libc::c_double = cos(6 as libc::c_int as libc::c_double * eta);
        let mut coeffs: *mut [[libc::c_double; 4]; 4] = 0 as *mut [[libc::c_double; 4]; 4];
        let mut safety: *mut libc::c_double = 0 as *mut libc::c_double;
        if degree == 2 as libc::c_int {
            coeffs = if x_0 < 0.25f64 {
                coeffs2Low.as_mut_ptr()
            } else {
                coeffs2High.as_mut_ptr()
            };
            safety = safety2.as_mut_ptr();
        } else {
            coeffs = if x_0 < 0.25f64 {
                coeffs3Low.as_mut_ptr()
            } else {
                coeffs3High.as_mut_ptr()
            };
            safety = safety3.as_mut_ptr();
        }
        c0 = (x_0
            * (x_0
                * (*coeffs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                    [0 as libc::c_int as usize]
                + (*coeffs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                    [1 as libc::c_int as usize])
            + (*coeffs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                [2 as libc::c_int as usize])
            / (x_0
                + (*coeffs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                    [3 as libc::c_int as usize])
            + cos2
                * ((x_0
                    * (x_0
                        * (*coeffs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
                            [0 as libc::c_int as usize]
                        + (*coeffs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
                            [1 as libc::c_int as usize])
                    + (*coeffs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
                        [2 as libc::c_int as usize])
                    / (x_0
                        + (*coeffs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
                            [3 as libc::c_int as usize]))
            + cos4
                * ((x_0
                    * (x_0
                        * (*coeffs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
                            [0 as libc::c_int as usize]
                        + (*coeffs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
                            [1 as libc::c_int as usize])
                    + (*coeffs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
                        [2 as libc::c_int as usize])
                    / (x_0
                        + (*coeffs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
                            [3 as libc::c_int as usize]))
            + cos6
                * ((x_0
                    * (x_0
                        * (*coeffs.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
                            [0 as libc::c_int as usize]
                        + (*coeffs.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
                            [1 as libc::c_int as usize])
                    + (*coeffs.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
                        [2 as libc::c_int as usize])
                    / (x_0
                        + (*coeffs.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
                            [3 as libc::c_int as usize]));
        c1 = (x_0
            * (x_0
                * (*coeffs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
                    [0 as libc::c_int as usize]
                + (*coeffs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
                    [1 as libc::c_int as usize])
            + (*coeffs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
                [2 as libc::c_int as usize])
            / (x_0
                + (*coeffs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
                    [3 as libc::c_int as usize])
            + cos2
                * ((x_0
                    * (x_0
                        * (*coeffs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
                            [0 as libc::c_int as usize]
                        + (*coeffs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
                            [1 as libc::c_int as usize])
                    + (*coeffs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
                        [2 as libc::c_int as usize])
                    / (x_0
                        + (*coeffs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
                            [3 as libc::c_int as usize]))
            + cos4
                * ((x_0
                    * (x_0
                        * (*coeffs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
                            [0 as libc::c_int as usize]
                        + (*coeffs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
                            [1 as libc::c_int as usize])
                    + (*coeffs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
                        [2 as libc::c_int as usize])
                    / (x_0
                        + (*coeffs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
                            [3 as libc::c_int as usize]))
            + cos6
                * ((x_0
                    * (x_0
                        * (*coeffs.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]
                            [0 as libc::c_int as usize]
                        + (*coeffs.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]
                            [1 as libc::c_int as usize])
                    + (*coeffs.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]
                        [2 as libc::c_int as usize])
                    / (x_0
                        + (*coeffs.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]
                            [3 as libc::c_int as usize]));
        return (x_0
            * (x_0 * *safety.offset(0 as libc::c_int as isize)
                + *safety.offset(1 as libc::c_int as isize))
            + *safety.offset(2 as libc::c_int as isize))
            / (x_0 + *safety.offset(3 as libc::c_int as isize))
            * (*ep).a
            * exp(c0 + c1 * dEta);
    };
}
static mut bufsize: libc::c_int = 0;
unsafe extern "C" fn moveTo(
    mut polypath: *mut Ppolyline_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
) {
    bufsize = 100 as libc::c_int;
    let ref mut fresh0 = (*polypath).ps;
    *fresh0 = gcalloc(
        bufsize as size_t,
        ::std::mem::size_of::<pointf>() as libc::c_ulong,
    ) as *mut pointf;
    (*((*polypath).ps).offset(0 as libc::c_int as isize)).x = x;
    (*((*polypath).ps).offset(0 as libc::c_int as isize)).y = y;
    (*polypath).pn = 1 as libc::c_int;
}
unsafe extern "C" fn curveTo(
    mut polypath: *mut Ppolyline_t,
    mut x1: libc::c_double,
    mut y1: libc::c_double,
    mut x2: libc::c_double,
    mut y2: libc::c_double,
    mut x3: libc::c_double,
    mut y3: libc::c_double,
) {
    if (*polypath).pn + 3 as libc::c_int >= bufsize {
        bufsize *= 2 as libc::c_int;
        let ref mut fresh1 = (*polypath).ps;
        *fresh1 = realloc(
            (*polypath).ps as *mut libc::c_void,
            (bufsize as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pointf>() as libc::c_ulong),
        ) as *mut Ppoint_t;
    }
    (*((*polypath).ps).offset((*polypath).pn as isize)).x = x1;
    let ref mut fresh2 = (*polypath).pn;
    let fresh3 = *fresh2;
    *fresh2 = *fresh2 + 1;
    (*((*polypath).ps).offset(fresh3 as isize)).y = y1;
    (*((*polypath).ps).offset((*polypath).pn as isize)).x = x2;
    let ref mut fresh4 = (*polypath).pn;
    let fresh5 = *fresh4;
    *fresh4 = *fresh4 + 1;
    (*((*polypath).ps).offset(fresh5 as isize)).y = y2;
    (*((*polypath).ps).offset((*polypath).pn as isize)).x = x3;
    let ref mut fresh6 = (*polypath).pn;
    let fresh7 = *fresh6;
    *fresh6 = *fresh6 + 1;
    (*((*polypath).ps).offset(fresh7 as isize)).y = y3;
}
unsafe extern "C" fn lineTo(
    mut polypath: *mut Ppolyline_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
) {
    let mut curp: pointf = *((*polypath).ps).offset(((*polypath).pn - 1 as libc::c_int) as isize);
    curveTo(polypath, curp.x, curp.y, x, y, x, y);
}
unsafe extern "C" fn endPath(mut polypath: *mut Ppolyline_t) {
    let mut p0: pointf = *((*polypath).ps).offset(0 as libc::c_int as isize);
    lineTo(polypath, p0.x, p0.y);
    let ref mut fresh8 = (*polypath).ps;
    *fresh8 = realloc(
        (*polypath).ps as *mut libc::c_void,
        ((*polypath).pn as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<pointf>() as libc::c_ulong),
    ) as *mut Ppoint_t;
    bufsize = 0 as libc::c_int;
}
unsafe extern "C" fn genEllipticPath(mut ep: *mut ellipse_t) -> *mut Ppolyline_t {
    let mut dEta: libc::c_double = 0.;
    let mut etaB: libc::c_double = 0.;
    let mut cosEtaB: libc::c_double = 0.;
    let mut sinEtaB: libc::c_double = 0.;
    let mut aCosEtaB: libc::c_double = 0.;
    let mut bSinEtaB: libc::c_double = 0.;
    let mut aSinEtaB: libc::c_double = 0.;
    let mut bCosEtaB: libc::c_double = 0.;
    let mut xB: libc::c_double = 0.;
    let mut yB: libc::c_double = 0.;
    let mut xBDot: libc::c_double = 0.;
    let mut yBDot: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut alpha: libc::c_double = 0.;
    let mut polypath: *mut Ppolyline_t =
        zmalloc(::std::mem::size_of::<Ppolyline_t>() as libc::c_ulong) as *mut Ppolyline_t;
    static mut THRESHOLD: libc::c_double = 0.00001f64;
    static mut DEGREE: libc::c_int = 3 as libc::c_int;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 1 as libc::c_int;
    while !found && n < 1024 as libc::c_int {
        let mut diffEta: libc::c_double = ((*ep).eta2 - (*ep).eta1) / n as libc::c_double;
        if diffEta <= 0.5f64 * 3.14159265358979323846f64 {
            let mut etaOne: libc::c_double = (*ep).eta1;
            found = 1 as libc::c_int != 0;
            i = 0 as libc::c_int;
            while found as libc::c_int != 0 && i < n {
                let mut etaA: libc::c_double = etaOne;
                etaOne += diffEta;
                found = estimateError(ep, DEGREE, etaA, etaOne) <= THRESHOLD;
                i += 1;
            }
        }
        n = n << 1 as libc::c_int;
    }
    dEta = ((*ep).eta2 - (*ep).eta1) / n as libc::c_double;
    etaB = (*ep).eta1;
    cosEtaB = cos(etaB);
    sinEtaB = sin(etaB);
    aCosEtaB = (*ep).a * cosEtaB;
    bSinEtaB = (*ep).b * sinEtaB;
    aSinEtaB = (*ep).a * sinEtaB;
    bCosEtaB = (*ep).b * cosEtaB;
    xB = (*ep).cx + aCosEtaB * (*ep).cosTheta - bSinEtaB * (*ep).sinTheta;
    yB = (*ep).cy + aCosEtaB * (*ep).sinTheta + bSinEtaB * (*ep).cosTheta;
    xBDot = -aSinEtaB * (*ep).cosTheta - bCosEtaB * (*ep).sinTheta;
    yBDot = -aSinEtaB * (*ep).sinTheta + bCosEtaB * (*ep).cosTheta;
    moveTo(polypath, (*ep).cx, (*ep).cy);
    lineTo(polypath, xB, yB);
    t = tan(0.5f64 * dEta);
    alpha = sin(dEta)
        * (sqrt(4 as libc::c_int as libc::c_double + 3 as libc::c_int as libc::c_double * t * t)
            - 1 as libc::c_int as libc::c_double)
        / 3 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        let mut xA: libc::c_double = xB;
        let mut yA: libc::c_double = yB;
        let mut xADot: libc::c_double = xBDot;
        let mut yADot: libc::c_double = yBDot;
        etaB += dEta;
        cosEtaB = cos(etaB);
        sinEtaB = sin(etaB);
        aCosEtaB = (*ep).a * cosEtaB;
        bSinEtaB = (*ep).b * sinEtaB;
        aSinEtaB = (*ep).a * sinEtaB;
        bCosEtaB = (*ep).b * cosEtaB;
        xB = (*ep).cx + aCosEtaB * (*ep).cosTheta - bSinEtaB * (*ep).sinTheta;
        yB = (*ep).cy + aCosEtaB * (*ep).sinTheta + bSinEtaB * (*ep).cosTheta;
        xBDot = -aSinEtaB * (*ep).cosTheta - bCosEtaB * (*ep).sinTheta;
        yBDot = -aSinEtaB * (*ep).sinTheta + bCosEtaB * (*ep).cosTheta;
        curveTo(
            polypath,
            xA + alpha * xADot,
            yA + alpha * yADot,
            xB - alpha * xBDot,
            yB - alpha * yBDot,
            xB,
            yB,
        );
        i += 1;
    }
    endPath(polypath);
    return polypath;
}
#[no_mangle]
pub unsafe extern "C" fn ellipticWedge(
    mut ctr: pointf,
    mut xsemi: libc::c_double,
    mut ysemi: libc::c_double,
    mut angle0: libc::c_double,
    mut angle1: libc::c_double,
) -> *mut Ppolyline_t {
    let mut ell: ellipse_t = ellipse_t {
        cx: 0.,
        cy: 0.,
        a: 0.,
        b: 0.,
        theta: 0.,
        cosTheta: 0.,
        sinTheta: 0.,
        eta1: 0.,
        eta2: 0.,
        x1: 0.,
        y1: 0.,
        x2: 0.,
        y2: 0.,
        xF1: 0.,
        yF1: 0.,
        xF2: 0.,
        yF2: 0.,
        xLeft: 0.,
        yUp: 0.,
        width: 0.,
        height: 0.,
        f: 0.,
        e2: 0.,
        g: 0.,
        g2: 0.,
    };
    let mut pp: *mut Ppolyline_t = 0 as *mut Ppolyline_t;
    initEllipse(
        &mut ell,
        ctr.x,
        ctr.y,
        xsemi,
        ysemi,
        0 as libc::c_int as libc::c_double,
        angle0,
        angle1,
    );
    pp = genEllipticPath(&mut ell);
    return pp;
}
