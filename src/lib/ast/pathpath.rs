#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn pathaccess(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn strcopy(s: *mut libc::c_char, t: *const libc::c_char) -> *mut libc::c_char;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
unsafe extern "C" fn getenv_path() -> *const libc::c_char {
    let mut path: *const libc::c_char = getenv(
        b"PATH\0" as *const u8 as *const libc::c_char,
    );
    if !path.is_null() {
        return path;
    }
    return b"\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub static mut opt_info_argv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn pathpath(
    mut path: *mut libc::c_char,
    mut p: *const libc::c_char,
) -> *mut libc::c_char {
    let mut a: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    static mut cmd: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    if path.is_null() {
        path = buf.as_mut_ptr();
    }
    if p.is_null() {
        free(cmd as *mut libc::c_void);
        cmd = if !a.is_null() { strdup(a) } else { 0 as *mut libc::c_char };
        return 0 as *mut libc::c_char;
    }
    if strlen(p) < 4096 as libc::c_int as libc::c_ulong {
        strcpy(path, p);
        let mut st: stat = stat {
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
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        if stat(path, &mut st) == 0 as libc::c_int
            && !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
        {
            return if path == buf.as_mut_ptr() { strdup(path) } else { path };
        }
    }
    if *p as libc::c_int == '/' as i32 {
        a = 0 as *const libc::c_char;
    } else {
        s = a as *mut libc::c_char;
        if !s.is_null() {
            x = s;
            if !(strchr(p, '/' as i32)).is_null() {
                a = p;
                p = b"..\0" as *const u8 as *const libc::c_char;
            } else {
                a = 0 as *const libc::c_char;
            }
            if (cmd.is_null() || *cmd as libc::c_int != 0)
                && (!(strchr(s, '/' as i32)).is_null()
                    || {
                        s = cmd;
                        (!s.is_null()
                            || !opt_info_argv.is_null()
                                && {
                                    s = *opt_info_argv;
                                    !s.is_null()
                                }) && !(strchr(s, '/' as i32)).is_null()
                            && (strchr(s, '\n' as i32)).is_null()
                            && access(s, 0 as libc::c_int) == 0
                    }
                    || {
                        s = getenv(b"_\0" as *const u8 as *const libc::c_char);
                        !s.is_null() && !(strchr(s, '/' as i32)).is_null()
                            && strncmp(
                                s,
                                b"/bin/\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int as libc::c_ulong,
                            ) != 0
                            && strncmp(
                                s,
                                b"/usr/bin/\0" as *const u8 as *const libc::c_char,
                                9 as libc::c_int as libc::c_ulong,
                            ) != 0
                    }
                    || *x as libc::c_int != 0 && access(x, 0 as libc::c_int) == 0
                        && {
                            s = getenv(b"PWD\0" as *const u8 as *const libc::c_char);
                            !s.is_null()
                        } && *s as libc::c_int == '/' as i32)
            {
                if cmd.is_null() {
                    cmd = strdup(s);
                }
                if strlen(s)
                    < (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                        .wrapping_sub(6 as libc::c_int as libc::c_ulong)
                {
                    s = strcopy(path, s);
                    's_135: while !(s <= path) {
                        s = s.offset(-1);
                        if *s as libc::c_int == '/' as i32 {
                            continue;
                        }
                        loop {
                            if s <= path {
                                break 's_135;
                            }
                            s = s.offset(-1);
                            if !(*s as libc::c_int != '/' as i32) {
                                break;
                            }
                        }
                        strcpy(
                            s.offset(1 as libc::c_int as isize),
                            b"bin\0" as *const u8 as *const libc::c_char,
                        );
                        if !(access(path, 1 as libc::c_int) == 0 as libc::c_int) {
                            continue;
                        }
                        s = pathaccess(path, path, p, a, 0o10 as libc::c_int);
                        if !s.is_null() {
                            return if path == buf.as_mut_ptr() { strdup(s) } else { s };
                        }
                        break;
                    }
                }
            }
        }
    }
    x = if a.is_null() && !(strchr(p, '/' as i32)).is_null() {
        b"\0" as *const u8 as *const libc::c_char
    } else {
        getenv_path()
    };
    s = pathaccess(path, x, p, a, 0o10 as libc::c_int);
    if s.is_null() && *x == 0
        && {
            x = getenv(b"FPATH\0" as *const u8 as *const libc::c_char);
            !x.is_null()
        }
    {
        s = pathaccess(path, x, p, a, 0o10 as libc::c_int);
    }
    return if !s.is_null() && path == buf.as_mut_ptr() { strdup(s) } else { s };
}
