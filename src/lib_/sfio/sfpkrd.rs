#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub const MSG_PEEK: C2RustUnnamed = 2;
pub type C2RustUnnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_OOB: C2RustUnnamed = 1;
#[no_mangle]
pub unsafe extern "C" fn sfpkrd(
    mut fd: libc::c_int,
    mut argbuf: *mut libc::c_void,
    mut n: size_t,
    mut rc: libc::c_int,
    mut tm: libc::c_long,
    mut action: libc::c_int,
) -> ssize_t {
    let mut r: ssize_t = 0;
    let mut ntry: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut buf: *mut libc::c_char = argbuf as *mut libc::c_char;
    let mut endbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    if rc < 0 as libc::c_int && tm < 0 as libc::c_int as libc::c_long
        && action <= 0 as libc::c_int
    {
        return read(fd, buf as *mut libc::c_void, n);
    }
    t = if action > 0 as libc::c_int || rc >= 0 as libc::c_int {
        0o1 as libc::c_int | 0o2 as libc::c_int
    } else {
        0 as libc::c_int
    };
    t &= !(0o1 as libc::c_int);
    ntry = 0 as libc::c_int;
    while ntry < 2 as libc::c_int {
        r = -(1 as libc::c_int) as ssize_t;
        if ntry == 1 as libc::c_int {
            break;
        }
        while tm >= 0 as libc::c_int as libc::c_long || action > 0 as libc::c_int
            || t & 0o1 as libc::c_int != 0 && rc >= 0 as libc::c_int
            || t & 0o2 as libc::c_int != 0
        {
            r = -(2 as libc::c_int) as ssize_t;
            if r == -(2 as libc::c_int) as libc::c_long {
                let mut rd: fd_set = fd_set { __fds_bits: [0; 16] };
                let mut tmb: timeval = timeval { tv_sec: 0, tv_usec: 0 };
                let mut tmp: *mut timeval = 0 as *mut timeval;
                let mut __i: libc::c_uint = 0;
                let mut __arr: *mut fd_set = &mut rd;
                __i = 0 as libc::c_int as libc::c_uint;
                while (__i as libc::c_ulong)
                    < (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<__fd_mask>() as libc::c_ulong,
                        )
                {
                    (*__arr).__fds_bits[__i as usize] = 0 as libc::c_int as __fd_mask;
                    __i = __i.wrapping_add(1);
                }
                rd
                    .__fds_bits[(fd
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    |= ((1 as libc::c_ulong)
                        << fd
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask;
                if tm < 0 as libc::c_int as libc::c_long {
                    tmp = 0 as *mut timeval;
                } else {
                    tmp = &mut tmb;
                    tmb.tv_sec = tm / 1000 as libc::c_int as libc::c_long;
                    tmb
                        .tv_usec = tm % 1000 as libc::c_int as libc::c_long
                        * 1000 as libc::c_int as libc::c_long;
                }
                r = select(
                    fd + 1 as libc::c_int,
                    &mut rd,
                    0 as *mut fd_set,
                    0 as *mut fd_set,
                    tmp,
                ) as ssize_t;
                if r < 0 as libc::c_int as libc::c_long {
                    if *__errno_location() == 4 as libc::c_int {
                        return -(1 as libc::c_int) as ssize_t
                    } else if *__errno_location() == 11 as libc::c_int {
                        *__errno_location() = 0 as libc::c_int;
                        continue;
                    } else {
                        r = -(2 as libc::c_int) as ssize_t;
                    }
                } else {
                    r = (if rd
                        .__fds_bits[(fd
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        & ((1 as libc::c_ulong)
                            << fd
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask
                        != 0 as libc::c_int as libc::c_long
                    {
                        1 as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    }) as ssize_t;
                }
            }
            r == -(2 as libc::c_int) as libc::c_long;
            if r > 0 as libc::c_int as libc::c_long {
                if action <= 0 as libc::c_int && rc < 0 as libc::c_int {
                    return read(fd, buf as *mut libc::c_void, n)
                } else {
                    r = -(1 as libc::c_int) as ssize_t;
                }
            } else if tm >= 0 as libc::c_int as libc::c_long {
                return -(1 as libc::c_int) as ssize_t
            } else {
                r = -(1 as libc::c_int) as ssize_t;
            }
            break;
        }
        if t & 0o2 as libc::c_int != 0 {
            loop {
                r = recv(fd, buf as *mut libc::c_void, n, MSG_PEEK as libc::c_int);
                if !(r < 0 as libc::c_int as libc::c_long) {
                    break;
                }
                if *__errno_location() == 4 as libc::c_int {
                    return -(1 as libc::c_int) as ssize_t
                } else if *__errno_location() == 11 as libc::c_int {
                    *__errno_location() = 0 as libc::c_int;
                } else {
                    t &= !(0o2 as libc::c_int);
                    break;
                }
            }
            if r >= 0 as libc::c_int as libc::c_long {
                t &= !(0o1 as libc::c_int);
                if r > 0 as libc::c_int as libc::c_long {
                    break;
                }
                if action <= 0 as libc::c_int {
                    r = read(fd, buf as *mut libc::c_void, 1 as libc::c_int as size_t);
                }
                return r;
            }
        }
        ntry += 1;
    }
    if r < 0 as libc::c_int as libc::c_long {
        if tm >= 0 as libc::c_int as libc::c_long || action > 0 as libc::c_int {
            return -(1 as libc::c_int) as ssize_t
        } else {
            action = (if action != 0 { -action } else { 1 as libc::c_int });
            if action > n as libc::c_int {
                action = n as libc::c_int;
            }
            r = 0 as libc::c_int as ssize_t;
            loop {
                t = read(fd, buf as *mut libc::c_void, action as size_t) as libc::c_int;
                if !(t > 0 as libc::c_int) {
                    break;
                }
                r += t as libc::c_long;
                endbuf = buf.offset(t as isize);
                while buf < endbuf {
                    let fresh0 = buf;
                    buf = buf.offset(1);
                    if *fresh0 as libc::c_int == rc {
                        action -= 1 as libc::c_int;
                    }
                }
                if action == 0 as libc::c_int
                    || (n.wrapping_sub(r as libc::c_ulong) as libc::c_int) < action
                {
                    break;
                }
            }
            return if r == 0 as libc::c_int as libc::c_long {
                t as libc::c_long
            } else {
                r
            };
        }
    }
    if rc >= 0 as libc::c_int {
        let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
        t = if action == 0 as libc::c_int {
            1 as libc::c_int
        } else if action < 0 as libc::c_int {
            -action
        } else {
            action
        };
        sp = buf;
        endbuf = sp.offset(r as isize);
        while sp < endbuf {
            let fresh1 = sp;
            sp = sp.offset(1);
            if !(*fresh1 as libc::c_int == rc) {
                continue;
            }
            t -= 1 as libc::c_int;
            if t == 0 as libc::c_int {
                break;
            }
        }
        r = sp.offset_from(buf) as libc::c_long;
    }
    if action <= 0 as libc::c_int {
        r = read(fd, buf as *mut libc::c_void, r as size_t);
    }
    return r;
}
