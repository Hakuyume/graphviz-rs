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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rk_state_ {
    pub key: [libc::c_ulong; 624],
    pub pos: libc::c_int,
}
pub type rk_state = rk_state_;
#[no_mangle]
pub unsafe extern "C" fn rk_seed(mut seed: libc::c_ulong, mut state: *mut rk_state) {
    let mut pos: libc::c_int = 0;
    seed &= 0xffffffff as libc::c_ulong;
    pos = 0 as libc::c_int;
    while pos < 624 as libc::c_int {
        (*state).key[pos as usize] = seed;
        seed = (1812433253 as libc::c_ulong)
            .wrapping_mul(seed ^ seed >> 30 as libc::c_int)
            .wrapping_add(pos as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            & 0xffffffff as libc::c_ulong;
        pos += 1;
    }
    (*state).pos = 624 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rk_random(mut state: *mut rk_state) -> libc::c_ulong {
    let mut y: libc::c_ulong = 0;
    if (*state).pos == 624 as libc::c_int {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 624 as libc::c_int - 397 as libc::c_int {
            y = (*state).key[i as usize] & 0x80000000 as libc::c_ulong
                | (*state).key[(i + 1 as libc::c_int) as usize] & 0x7fffffff as libc::c_ulong;
            (*state).key[i as usize] = (*state).key[(i + 397 as libc::c_int) as usize]
                ^ y >> 1 as libc::c_int
                ^ (y & 1 as libc::c_int as libc::c_ulong).wrapping_neg()
                    & 0x9908b0df as libc::c_ulong;
            i += 1;
        }
        while i < 624 as libc::c_int - 1 as libc::c_int {
            y = (*state).key[i as usize] & 0x80000000 as libc::c_ulong
                | (*state).key[(i + 1 as libc::c_int) as usize] & 0x7fffffff as libc::c_ulong;
            (*state).key[i as usize] = (*state).key
                [(i + (397 as libc::c_int - 624 as libc::c_int)) as usize]
                ^ y >> 1 as libc::c_int
                ^ (y & 1 as libc::c_int as libc::c_ulong).wrapping_neg()
                    & 0x9908b0df as libc::c_ulong;
            i += 1;
        }
        y = (*state).key[(624 as libc::c_int - 1 as libc::c_int) as usize]
            & 0x80000000 as libc::c_ulong
            | (*state).key[0 as libc::c_int as usize] & 0x7fffffff as libc::c_ulong;
        (*state).key[(624 as libc::c_int - 1 as libc::c_int) as usize] = (*state).key
            [(397 as libc::c_int - 1 as libc::c_int) as usize]
            ^ y >> 1 as libc::c_int
            ^ (y & 1 as libc::c_int as libc::c_ulong).wrapping_neg() & 0x9908b0df as libc::c_ulong;
        (*state).pos = 0 as libc::c_int;
    }
    let ref mut fresh0 = (*state).pos;
    let fresh1 = *fresh0;
    *fresh0 = *fresh0 + 1;
    y = (*state).key[fresh1 as usize];
    y ^= y >> 11 as libc::c_int;
    y ^= y << 7 as libc::c_int & 0x9d2c5680 as libc::c_ulong;
    y ^= y << 15 as libc::c_int & 0xefc60000 as libc::c_ulong;
    y ^= y >> 18 as libc::c_int;
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn rk_ulong(mut state: *mut rk_state) -> libc::c_ulong {
    return rk_random(state) << 32 as libc::c_int | rk_random(state);
}
#[no_mangle]
pub unsafe extern "C" fn rk_interval(
    mut max: libc::c_ulong,
    mut state: *mut rk_state,
) -> libc::c_ulong {
    let mut mask: libc::c_ulong = max;
    let mut value: libc::c_ulong = 0;
    if max == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_ulong;
    }
    mask |= mask >> 1 as libc::c_int;
    mask |= mask >> 2 as libc::c_int;
    mask |= mask >> 4 as libc::c_int;
    mask |= mask >> 8 as libc::c_int;
    mask |= mask >> 16 as libc::c_int;
    mask |= mask >> 32 as libc::c_int;
    if max <= 0xffffffff as libc::c_ulong {
        loop {
            value = rk_random(state) & mask;
            if !(value > max) {
                break;
            }
        }
    } else {
        loop {
            value = rk_ulong(state) & mask;
            if !(value > max) {
                break;
            }
        }
    }
    return value;
}
