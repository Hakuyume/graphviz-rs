#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
pub type __int32_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Group_t {
    pub beg: [*mut libc::c_char; 10],
    pub end: [*mut libc::c_char; 10],
    pub next_s: *mut libc::c_char,
    pub groups: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Match_t {
    pub current: Group_t,
    pub best: Group_t,
    pub last_s: *mut libc::c_char,
    pub next_p: *mut libc::c_char,
}
pub const _ISupper: C2RustUnnamed = 256;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISalnum: C2RustUnnamed = 8;
pub type C2RustUnnamed = libc::c_uint;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn gobble(
    mut mp: *mut Match_t,
    mut s: *mut libc::c_char,
    mut sub: libc::c_int,
    mut g: *mut libc::c_int,
    mut clear: libc::c_int,
) -> *mut libc::c_char {
    let mut p: libc::c_int = 0 as libc::c_int;
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    loop {
        let mut current_block_27: u64;
        let fresh0 = s;
        s = s.offset(1);
        match *fresh0 as libc::c_int {
            92 => {
                let fresh1 = s;
                s = s.offset(1);
                if *fresh1 != 0 {
                    current_block_27 = 2873832966593178012;
                } else {
                    current_block_27 = 6129774855485884492;
                }
            }
            0 => {
                current_block_27 = 6129774855485884492;
            }
            91 => {
                if b.is_null() {
                    if *s as libc::c_int == '!' as i32 {
                        let fresh2 = s;
                        s = s.offset(1);
                    }
                    b = s;
                } else if *s as libc::c_int == '.' as i32
                        || *s as libc::c_int == '=' as i32
                        || *s as libc::c_int == ':' as i32
                    {
                    c = *s as libc::c_int;
                }
                current_block_27 = 2873832966593178012;
            }
            93 => {
                if !b.is_null() {
                    if *s.offset(-(2 as libc::c_int as isize)) as libc::c_int == c {
                        c = 0 as libc::c_int;
                    } else if b != s.offset(-(1 as libc::c_int as isize)) {
                        b = 0 as *mut libc::c_char;
                    }
                }
                current_block_27 = 2873832966593178012;
            }
            40 => {
                if b.is_null() {
                    p += 1;
                    let fresh3 = *g;
                    *g = *g + 1;
                    n = fresh3;
                    if clear != 0 {
                        if sub == 0 {
                            n += 1;
                        }
                        if n < 10 as libc::c_int {
                            let ref mut fresh4 = (*mp).current.end[n as usize];
                            *fresh4 = 0 as *mut libc::c_char;
                            let ref mut fresh5 = (*mp).current.beg[n as usize];
                            *fresh5 = *fresh4;
                        }
                    }
                }
                current_block_27 = 2873832966593178012;
            }
            41 => {
                if b.is_null()
                    && {
                        let fresh6 = p;
                        p = p - 1;
                        fresh6 <= 0 as libc::c_int
                    }
                {
                    return if sub != 0 { 0 as *mut libc::c_char } else { s };
                }
                current_block_27 = 2873832966593178012;
            }
            124 => {
                if b.is_null() && p == 0 && sub == '|' as i32 {
                    return s;
                }
                current_block_27 = 2873832966593178012;
            }
            _ => {
                current_block_27 = 2873832966593178012;
            }
        }
        match current_block_27 {
            2873832966593178012 => {}
            _ => return 0 as *mut libc::c_char,
        }
    };
}
unsafe extern "C" fn onematch(
    mut mp: *mut Match_t,
    mut g: libc::c_int,
    mut s: *mut libc::c_char,
    mut p: *mut libc::c_char,
    mut e: *mut libc::c_char,
    mut r: *mut libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut pc: libc::c_int = 0;
    let mut sc: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut icase: libc::c_int = 0;
    let mut olds: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oldp: *mut libc::c_char = 0 as *mut libc::c_char;
    icase = flags & 0o10 as libc::c_int;
    loop {
        olds = s;
        sc = if s >= e {
            0 as libc::c_int
        } else {
            let fresh7 = s;
            s = s.offset(1);
            *fresh7 as libc::c_int
        };
        if icase != 0
            && *(*__ctype_b_loc()).offset(sc as isize) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            sc = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = sc;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(sc);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(sc as isize);
                }
                __res
            });
        }
        oldp = p;
        let mut current_block_231: u64;
        let fresh8 = p;
        p = p.offset(1);
        pc = *fresh8 as libc::c_int;
        match pc {
            40 | 42 | 63 | 43 | 64 | 33 => {
                if pc == '(' as i32 || *p as libc::c_int == '(' as i32 {
                    let mut subp: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut oldg: libc::c_int = 0;
                    s = olds;
                    subp = p.offset((pc != '(' as i32) as libc::c_int as isize);
                    oldg = g;
                    g += 1;
                    n = g;
                    if g < 10 as libc::c_int
                        && (r.is_null() || g > (*mp).current.groups as libc::c_int)
                    {
                        let ref mut fresh9 = (*mp).current.end[g as usize];
                        *fresh9 = 0 as *mut libc::c_char;
                        let ref mut fresh10 = (*mp).current.beg[g as usize];
                        *fresh10 = *fresh9;
                    }
                    p = gobble(
                        mp,
                        subp,
                        0 as libc::c_int,
                        &mut g,
                        r.is_null() as libc::c_int,
                    );
                    if p.is_null() {
                        return 0 as libc::c_int;
                    }
                    if pc == '*' as i32 || pc == '?' as i32
                        || pc == '+' as i32 && oldp == r
                    {
                        if onematch(mp, g, s, p, e, 0 as *mut libc::c_char, flags) != 0 {
                            return 1 as libc::c_int;
                        }
                        if sc == 0
                            || (if s >= e {
                                0 as libc::c_int
                            } else {
                                let fresh11 = s;
                                s = s.offset(1);
                                *fresh11 as libc::c_int
                            }) == 0
                        {
                            (*mp).current.groups = oldg as libc::c_short;
                            return 0 as libc::c_int;
                        }
                    }
                    if pc == '*' as i32 || pc == '+' as i32 {
                        p = oldp;
                        sc = n - 1 as libc::c_int;
                    } else {
                        sc = g;
                    }
                    pc = (pc != '!' as i32) as libc::c_int;
                    loop {
                        if grpmatch(mp, n, olds, subp, s, flags) == pc {
                            if n < 10 as libc::c_int {
                                if ((*mp).current.beg[n as usize]).is_null()
                                    || (*mp).current.beg[n as usize] > olds
                                {
                                    let ref mut fresh12 = (*mp).current.beg[n as usize];
                                    *fresh12 = olds;
                                }
                                if s > (*mp).current.end[n as usize] {
                                    let ref mut fresh13 = (*mp).current.end[n as usize];
                                    *fresh13 = s;
                                }
                            }
                            if onematch(mp, sc, s, p, e, oldp, flags) != 0 {
                                if p == oldp && n < 10 as libc::c_int {
                                    if ((*mp).current.beg[n as usize]).is_null()
                                        || (*mp).current.beg[n as usize] > olds
                                    {
                                        let ref mut fresh14 = (*mp).current.beg[n as usize];
                                        *fresh14 = olds;
                                    }
                                    if s > (*mp).current.end[n as usize] {
                                        let ref mut fresh15 = (*mp).current.end[n as usize];
                                        *fresh15 = s;
                                    }
                                }
                                return 1 as libc::c_int;
                            }
                        }
                        if !(s < e
                            && {
                                let fresh16 = s;
                                s = s.offset(1);
                                *fresh16 as libc::c_int != 0
                            })
                        {
                            break;
                        }
                    }
                    (*mp).current.groups = oldg as libc::c_short;
                    return 0 as libc::c_int;
                } else {
                    if pc == '*' as i32 {
                        while *p as libc::c_int == '*' as i32
                            && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                != '(' as i32
                        {
                            p = p.offset(1);
                        }
                        oldp = p;
                        let mut current_block_85: u64;
                        let fresh17 = p;
                        p = p.offset(1);
                        pc = *fresh17 as libc::c_int;
                        match pc {
                            64 | 33 | 43 => {
                                n = (*p as libc::c_int == '(' as i32) as libc::c_int;
                                current_block_85 = 2750570471926810434;
                            }
                            40 | 91 | 63 | 42 => {
                                n = 1 as libc::c_int;
                                current_block_85 = 2750570471926810434;
                            }
                            0 | 124 | 38 | 41 => {
                                let ref mut fresh18 = (*mp).current.next_s;
                                *fresh18 = if flags & 0o1 as libc::c_int != 0 {
                                    e
                                } else {
                                    olds
                                };
                                let ref mut fresh19 = (*mp).next_p;
                                *fresh19 = oldp;
                                (*mp).current.groups = g as libc::c_short;
                                if pc == 0
                                    && (((*mp).best.next_s).is_null()
                                        || flags & 0o1 as libc::c_int != 0
                                            && (*mp).current.next_s > (*mp).best.next_s
                                        || flags & 0o1 as libc::c_int == 0
                                            && (*mp).current.next_s < (*mp).best.next_s)
                                {
                                    (*mp).best = (*mp).current;
                                }
                                return 1 as libc::c_int;
                            }
                            92 => {
                                let fresh20 = p;
                                p = p.offset(1);
                                pc = *fresh20 as libc::c_int;
                                if pc == 0 {
                                    return 0 as libc::c_int;
                                }
                                if pc >= '0' as i32 && pc <= '9' as i32 {
                                    n = pc - '0' as i32;
                                    if n <= g && !((*mp).current.beg[n as usize]).is_null() {
                                        pc = *(*mp).current.beg[n as usize] as libc::c_int;
                                    }
                                }
                                current_block_85 = 13488985058286422962;
                            }
                            _ => {
                                current_block_85 = 13488985058286422962;
                            }
                        }
                        match current_block_85 {
                            13488985058286422962 => {
                                if icase != 0
                                    && *(*__ctype_b_loc()).offset(pc as isize) as libc::c_int
                                        & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                                        != 0
                                {
                                    pc = ({
                                        let mut __res: libc::c_int = 0;
                                        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            > 1 as libc::c_int as libc::c_ulong
                                        {
                                            if 0 != 0 {
                                                let mut __c: libc::c_int = pc;
                                                __res = if __c < -(128 as libc::c_int)
                                                    || __c > 255 as libc::c_int
                                                {
                                                    __c
                                                } else {
                                                    *(*__ctype_tolower_loc()).offset(__c as isize)
                                                };
                                            } else {
                                                __res = tolower(pc);
                                            }
                                        } else {
                                            __res = *(*__ctype_tolower_loc()).offset(pc as isize);
                                        }
                                        __res
                                    });
                                }
                                n = 0 as libc::c_int;
                            }
                            _ => {}
                        }
                        p = oldp;
                        loop {
                            if (n != 0 || pc == sc)
                                && onematch(
                                    mp,
                                    g,
                                    olds,
                                    p,
                                    e,
                                    0 as *mut libc::c_char,
                                    flags,
                                ) != 0
                            {
                                return 1 as libc::c_int;
                            }
                            if sc == 0 {
                                return 0 as libc::c_int;
                            }
                            olds = s;
                            sc = if s >= e {
                                0 as libc::c_int
                            } else {
                                let fresh21 = s;
                                s = s.offset(1);
                                *fresh21 as libc::c_int
                            };
                            if flags & 0o10 as libc::c_int != 0
                                && *(*__ctype_b_loc()).offset(sc as isize) as libc::c_int
                                    & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                            {
                                sc = ({
                                    let mut __res: libc::c_int = 0;
                                    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        > 1 as libc::c_int as libc::c_ulong
                                    {
                                        if 0 != 0 {
                                            let mut __c: libc::c_int = sc;
                                            __res = if __c < -(128 as libc::c_int)
                                                || __c > 255 as libc::c_int
                                            {
                                                __c
                                            } else {
                                                *(*__ctype_tolower_loc()).offset(__c as isize)
                                            };
                                        } else {
                                            __res = tolower(sc);
                                        }
                                    } else {
                                        __res = *(*__ctype_tolower_loc()).offset(sc as isize);
                                    }
                                    __res
                                });
                            }
                        }
                    } else {
                        if pc != '?' as i32 && pc != sc {
                            return 0 as libc::c_int;
                        }
                    }
                }
                current_block_231 = 6813271534392596583;
            }
            0 => {
                if flags & 0o1 as libc::c_int == 0 {
                    sc = 0 as libc::c_int;
                }
                current_block_231 = 1875299150326674628;
            }
            124 | 38 | 41 => {
                current_block_231 = 1875299150326674628;
            }
            91 => {
                let mut invert: libc::c_int = 0;
                let mut x: libc::c_int = 0;
                let mut ok: libc::c_int = 0 as libc::c_int;
                let mut range: *mut libc::c_char = 0 as *mut libc::c_char;
                if sc == 0 {
                    return 0 as libc::c_int;
                }
                range = 0 as *mut libc::c_char;
                n = 0 as libc::c_int;
                invert = (*p as libc::c_int == '!' as i32) as libc::c_int;
                if invert != 0 {
                    p = p.offset(1);
                }
                let mut current_block_212: u64;
                loop {
                    oldp = p;
                    let fresh25 = p;
                    p = p.offset(1);
                    pc = *fresh25 as libc::c_int;
                    if pc == 0 {
                        return 0 as libc::c_int
                    } else {
                        if pc == '[' as i32
                            && (*p as libc::c_int == ':' as i32
                                || *p as libc::c_int == '=' as i32
                                || *p as libc::c_int == '.' as i32)
                        {
                            x = 0 as libc::c_int;
                            let fresh26 = p;
                            p = p.offset(1);
                            n = *fresh26 as libc::c_int;
                            oldp = p;
                            loop {
                                let fresh27 = p;
                                p = p.offset(1);
                                pc = *fresh27 as libc::c_int;
                                if pc == 0 {
                                    return 0 as libc::c_int;
                                }
                                if pc == n && *p as libc::c_int == ']' as i32 {
                                    break;
                                }
                                x += 1;
                            }
                            let fresh28 = p;
                            p = p.offset(1);
                            if ok != 0 {
                                current_block_212 = 6958737826328148217;
                            } else if n == ':' as i32 {
                                match (((((x + 'a' as i32 - ('a' as i32 - 1 as libc::c_int)
                                    << 5 as libc::c_int)
                                    + (*oldp.offset(0 as libc::c_int as isize) as libc::c_int
                                        - ('a' as i32 - 1 as libc::c_int)) << 5 as libc::c_int)
                                    + (*oldp.offset(1 as libc::c_int as isize) as libc::c_int
                                        - ('a' as i32 - 1 as libc::c_int)) << 5 as libc::c_int)
                                    + (*oldp.offset(2 as libc::c_int as isize) as libc::c_int
                                        - ('a' as i32 - 1 as libc::c_int)) << 5 as libc::c_int)
                                    + (*oldp.offset(3 as libc::c_int as isize) as libc::c_int
                                        - ('a' as i32 - 1 as libc::c_int)) << 5 as libc::c_int)
                                    + (*oldp.offset(4 as libc::c_int as isize) as libc::c_int
                                        - ('a' as i32 - 1 as libc::c_int))
                                {
                                    202783405 => {
                                        if *(*__ctype_b_loc()).offset(sc as isize) as libc::c_int
                                            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                        {
                                            ok = 1 as libc::c_int;
                                        }
                                    }
                                    202785025 => {
                                        if *(*__ctype_b_loc()).offset(sc as isize) as libc::c_int
                                            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                        {
                                            ok = 1 as libc::c_int;
                                        }
                                    }
                                    203818443 => {
                                        if *(*__ctype_b_loc()).offset(sc as isize) as libc::c_int
                                            & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                        {
                                            ok = 1 as libc::c_int;
                                        }
                                    }
                                    204952140 => {
                                        if *(*__ctype_b_loc()).offset(sc as isize) as libc::c_int
                                            & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                        {
                                            ok = 1 as libc::c_int;
                                        }
                                    }
                                    205823284 => {
                                        if *(*__ctype_b_loc()).offset(sc as isize) as libc::c_int
                                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                        {
                                            ok = 1 as libc::c_int;
                                        }
                                    }
                                    209257992 => {
                                        if *(*__ctype_b_loc()).offset(sc as isize) as libc::c_int
                                            & _ISgraph as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                        {
                                            ok = 1 as libc::c_int;
                                        }
                                    }
                                    214424754 => {
                                        if *(*__ctype_b_loc()).offset(sc as isize) as libc::c_int
                                            & _ISlower as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                        {
                                            ok = 1 as libc::c_int;
                                        }
                                    }
                                    218703316 => {
                                        if *(*__ctype_b_loc()).offset(sc as isize) as libc::c_int
                                            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                        {
                                            ok = 1 as libc::c_int;
                                        }
                                    }
                                    218806388 => {
                                        if *(*__ctype_b_loc()).offset(sc as isize) as libc::c_int
                                            & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                        {
                                            ok = 1 as libc::c_int;
                                        }
                                    }
                                    221774949 => {
                                        if *(*__ctype_b_loc()).offset(sc as isize) as libc::c_int
                                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                        {
                                            ok = 1 as libc::c_int;
                                        }
                                    }
                                    223887538 => {
                                        if if icase != 0 {
                                            *(*__ctype_b_loc()).offset(sc as isize) as libc::c_int
                                                & _ISlower as libc::c_int as libc::c_ushort as libc::c_int
                                        } else {
                                            *(*__ctype_b_loc()).offset(sc as isize) as libc::c_int
                                                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                                        } != 0
                                        {
                                            ok = 1 as libc::c_int;
                                        }
                                    }
                                    260187369 => {
                                        if *oldp.offset(5 as libc::c_int as isize) as libc::c_int
                                            == 't' as i32
                                            && *(*__ctype_b_loc()).offset(sc as isize) as libc::c_int
                                                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                                                != 0
                                        {
                                            ok = 1 as libc::c_int;
                                        }
                                    }
                                    _ => {}
                                }
                                current_block_212 = 6958737826328148217;
                            } else if !range.is_null() {
                                current_block_212 = 780924439199517896;
                            } else {
                                if *p as libc::c_int == '-' as i32
                                    && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                        != ']' as i32
                                {
                                    let fresh29 = p;
                                    p = p.offset(1);
                                    range = oldp;
                                } else if *(*__ctype_b_loc())
                                        .offset(*oldp as libc::c_int as isize) as libc::c_int
                                        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                                        != 0
                                        && *(*__ctype_b_loc()).offset(*olds as libc::c_int as isize)
                                            as libc::c_int
                                            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                        && ({
                                            let mut __res: libc::c_int = 0;
                                            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                                > 1 as libc::c_int as libc::c_ulong
                                            {
                                                if 0 != 0 {
                                                    let mut __c: libc::c_int = *oldp as libc::c_int;
                                                    __res = (if __c < -(128 as libc::c_int)
                                                        || __c > 255 as libc::c_int
                                                    {
                                                        __c
                                                    } else {
                                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                                    });
                                                } else {
                                                    __res = tolower(*oldp as libc::c_int);
                                                }
                                            } else {
                                                __res = *(*__ctype_tolower_loc())
                                                    .offset(*oldp as libc::c_int as isize);
                                            }
                                            __res
                                        })
                                            == ({
                                                let mut __res: libc::c_int = 0;
                                                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                                    > 1 as libc::c_int as libc::c_ulong
                                                {
                                                    if 0 != 0 {
                                                        let mut __c: libc::c_int = *olds as libc::c_int;
                                                        __res = (if __c < -(128 as libc::c_int)
                                                            || __c > 255 as libc::c_int
                                                        {
                                                            __c
                                                        } else {
                                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                                        });
                                                    } else {
                                                        __res = tolower(*olds as libc::c_int);
                                                    }
                                                } else {
                                                    __res = *(*__ctype_tolower_loc())
                                                        .offset(*olds as libc::c_int as isize);
                                                }
                                                __res
                                            })
                                        || {
                                            let fresh30 = oldp;
                                            oldp = oldp.offset(1);
                                            sc == *fresh30 as libc::c_int
                                        }
                                    {
                                    ok = 1 as libc::c_int;
                                }
                                current_block_212 = 6958737826328148217;
                            }
                            match current_block_212 {
                                780924439199517896 => {}
                                _ => {
                                    n = 1 as libc::c_int;
                                    continue;
                                }
                            }
                        } else if pc == ']' as i32 && n != 0 {
                            if ok != invert {
                                break;
                            }
                            return 0 as libc::c_int;
                        } else if pc == '\\' as i32
                                && {
                                    oldp = p;
                                    let fresh31 = p;
                                    p = p.offset(1);
                                    pc = *fresh31 as libc::c_int;
                                    pc == 0
                                }
                            {
                            return 0 as libc::c_int
                        } else {
                            if ok != 0 {
                                continue;
                            }
                            if range.is_null() {
                                if *p as libc::c_int == '-' as i32
                                    && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                        != ']' as i32
                                {
                                    let fresh34 = p;
                                    p = p.offset(1);
                                    range = oldp;
                                    n = 1 as libc::c_int;
                                } else {
                                    if icase != 0
                                        && *(*__ctype_b_loc()).offset(pc as isize) as libc::c_int
                                            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                    {
                                        pc = ({
                                            let mut __res: libc::c_int = 0;
                                            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                > 1 as libc::c_int as libc::c_ulong
                                            {
                                                if 0 != 0 {
                                                    let mut __c: libc::c_int = pc;
                                                    __res = if __c < -(128 as libc::c_int)
                                                        || __c > 255 as libc::c_int
                                                    {
                                                        __c
                                                    } else {
                                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                                    };
                                                } else {
                                                    __res = tolower(pc);
                                                }
                                            } else {
                                                __res = *(*__ctype_tolower_loc()).offset(pc as isize);
                                            }
                                            __res
                                        });
                                    }
                                    if sc == pc {
                                        ok = 1 as libc::c_int;
                                    }
                                    n = pc;
                                }
                                continue;
                            }
                        }
                        if icase != 0
                            && *(*__ctype_b_loc()).offset(pc as isize) as libc::c_int
                                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                        {
                            pc = ({
                                let mut __res: libc::c_int = 0;
                                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = pc;
                                        __res = if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        };
                                    } else {
                                        __res = tolower(pc);
                                    }
                                } else {
                                    __res = *(*__ctype_tolower_loc()).offset(pc as isize);
                                }
                                __res
                            });
                        }
                        let fresh32 = range;
                        range = range.offset(1);
                        x = *fresh32 as libc::c_int;
                        if icase != 0
                            && *(*__ctype_b_loc()).offset(x as isize) as libc::c_int
                                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                        {
                            x = ({
                                let mut __res: libc::c_int = 0;
                                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = x;
                                        __res = if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        };
                                    } else {
                                        __res = tolower(x);
                                    }
                                } else {
                                    __res = *(*__ctype_tolower_loc()).offset(x as isize);
                                }
                                __res
                            });
                        }
                        if sc == x || sc == pc || sc > x && sc < pc {
                            ok = 1 as libc::c_int;
                        }
                        if *p as libc::c_int == '-' as i32
                            && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                != ']' as i32
                        {
                            let fresh33 = p;
                            p = p.offset(1);
                            range = oldp;
                        } else {
                            range = 0 as *mut libc::c_char;
                        }
                        n = 1 as libc::c_int;
                    }
                }
                current_block_231 = 6813271534392596583;
            }
            92 => {
                let fresh35 = p;
                p = p.offset(1);
                pc = *fresh35 as libc::c_int;
                if pc == 0 {
                    return 0 as libc::c_int;
                }
                if pc >= '0' as i32 && pc <= '9' as i32 {
                    n = pc - '0' as i32;
                    if n <= g
                        && {
                            oldp = (*mp).current.beg[n as usize];
                            !oldp.is_null()
                        }
                    {
                        while oldp < (*mp).current.end[n as usize] {
                            if *olds == 0
                                || {
                                    let fresh36 = olds;
                                    olds = olds.offset(1);
                                    let fresh37 = oldp;
                                    oldp = oldp.offset(1);
                                    *fresh36 as libc::c_int != *fresh37 as libc::c_int
                                }
                            {
                                return 0 as libc::c_int;
                            }
                        }
                        s = olds;
                        current_block_231 = 6813271534392596583;
                    } else {
                        current_block_231 = 14970271268288140784;
                    }
                } else {
                    current_block_231 = 14970271268288140784;
                }
            }
            _ => {
                current_block_231 = 14970271268288140784;
            }
        }
        match current_block_231 {
            14970271268288140784 => {
                if icase != 0
                    && *(*__ctype_b_loc()).offset(pc as isize) as libc::c_int
                        & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    pc = ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = pc;
                                __res = if __c < -(128 as libc::c_int)
                                    || __c > 255 as libc::c_int
                                {
                                    __c
                                } else {
                                    *(*__ctype_tolower_loc()).offset(__c as isize)
                                };
                            } else {
                                __res = tolower(pc);
                            }
                        } else {
                            __res = *(*__ctype_tolower_loc()).offset(pc as isize);
                        }
                        __res
                    });
                }
                if pc != sc {
                    return 0 as libc::c_int;
                }
            }
            1875299150326674628 => {
                if sc == 0 {
                    let ref mut fresh22 = (*mp).current.next_s;
                    *fresh22 = olds;
                    let ref mut fresh23 = (*mp).next_p;
                    *fresh23 = oldp;
                    (*mp).current.groups = g as libc::c_short;
                }
                if pc == 0
                    && (((*mp).best.next_s).is_null()
                        || flags & 0o1 as libc::c_int != 0 && olds > (*mp).best.next_s
                        || flags & 0o1 as libc::c_int == 0 && olds < (*mp).best.next_s)
                {
                    (*mp).best = (*mp).current;
                    let ref mut fresh24 = (*mp).best.next_s;
                    *fresh24 = olds;
                    (*mp).best.groups = g as libc::c_short;
                }
                return (sc == 0) as libc::c_int;
            }
            _ => {}
        }
        if !(sc != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn grpmatch(
    mut mp: *mut Match_t,
    mut g: libc::c_int,
    mut s: *mut libc::c_char,
    mut p: *mut libc::c_char,
    mut e: *mut libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        a = p;
        while onematch(mp, g, s, a, e, 0 as *mut libc::c_char, flags) != 0 {
            a = (*mp).next_p;
            if *a as libc::c_int != '&' as i32 {
                return 1 as libc::c_int;
            }
            a = a.offset(1);
        }
        p = gobble(mp, p, '|' as i32, &mut g, 1 as libc::c_int);
        if p.is_null() {
            break;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn strgrpmatch(
    mut b: *const libc::c_char,
    mut p: *const libc::c_char,
    mut sub: *mut libc::c_int,
    mut n: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut match_0: Match_t = Match_t {
        current: Group_t {
            beg: [0 as *mut libc::c_char; 10],
            end: [0 as *mut libc::c_char; 10],
            next_s: 0 as *mut libc::c_char,
            groups: 0,
        },
        best: Group_t {
            beg: [0 as *mut libc::c_char; 10],
            end: [0 as *mut libc::c_char; 10],
            next_s: 0 as *mut libc::c_char,
            groups: 0,
        },
        last_s: 0 as *mut libc::c_char,
        next_p: 0 as *mut libc::c_char,
    };
    s = b as *mut libc::c_char;
    e = s.offset(strlen(s) as isize);
    match_0.last_s = e;
    loop {
        match_0.best.next_s = 0 as *mut libc::c_char;
        match_0.current.groups = 0 as libc::c_int as libc::c_short;
        match_0.current.beg[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
        i = grpmatch(
            &mut match_0,
            0 as libc::c_int,
            s,
            p as *mut libc::c_char,
            e,
            flags,
        );
        if i != 0 || !(match_0.best.next_s).is_null() {
            if flags & 0o4 as libc::c_int == 0 || match_0.current.next_s == e {
                if i == 0 {
                    match_0.current = match_0.best;
                }
                match_0.current.groups += 1;
                match_0.current.end[0 as libc::c_int as usize] = match_0.current.next_s;
                break;
            }
        }
        if flags & 0o2 as libc::c_int != 0 || s >= e {
            return 0 as libc::c_int;
        }
        s = s.offset(1);
    }
    if flags & 0o4 as libc::c_int != 0 && match_0.current.next_s != e {
        return 0 as libc::c_int;
    }
    if sub.is_null() {
        return 1 as libc::c_int;
    }
    match_0.current.beg[0 as libc::c_int as usize] = s;
    s = b as *mut libc::c_char;
    if n > match_0.current.groups as libc::c_int {
        n = match_0.current.groups as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < n {
        *sub
            .offset(
                (i * 2 as libc::c_int) as isize,
            ) = (if !(match_0.current.end[i as usize]).is_null() {
            (match_0.current.beg[i as usize]).offset_from(s) as libc::c_long
        } else {
            0 as libc::c_int as libc::c_long
        }) as libc::c_int;
        *sub
            .offset(
                (i * 2 as libc::c_int + 1 as libc::c_int) as isize,
            ) = (if !(match_0.current.end[i as usize]).is_null() {
            (match_0.current.end[i as usize]).offset_from(s) as libc::c_long
        } else {
            0 as libc::c_int as libc::c_long
        }) as libc::c_int;
        i += 1;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn strmatch(
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
) -> libc::c_int {
    return strgrpmatch(
        s,
        p,
        0 as *mut libc::c_int,
        0 as libc::c_int,
        0o1 as libc::c_int | 0o2 as libc::c_int | 0o4 as libc::c_int,
    );
}
