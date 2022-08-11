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
    fn pathcat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: *const libc::c_char,
    ) -> *const libc::c_char;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn pathcanon(path: *mut libc::c_char) -> *mut libc::c_char;
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
pub type size_t = libc::c_ulong;
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
#[no_mangle]
pub unsafe extern "C" fn pathaccess(
    mut path: *mut libc::c_char,
    mut dirs: *const libc::c_char,
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
    mut mode: libc::c_int,
) -> *mut libc::c_char {
    let mut m: libc::c_int = 0 as libc::c_int;
    let mut sep: libc::c_int = ':' as i32;
    let mut cwd: [libc::c_char; 4096] = [0; 4096];
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
    if mode & 0o4 as libc::c_int != 0 {
        m |= 4 as libc::c_int;
    }
    if mode & 0o2 as libc::c_int != 0 {
        m |= 2 as libc::c_int;
    }
    if mode & 0o1 as libc::c_int != 0 {
        m |= 1 as libc::c_int;
    }
    loop {
        dirs = pathcat(path, dirs, sep, a, b);
        pathcanon(path);
        if access(path, m) == 0 {
            if !(mode & 0o10 as libc::c_int != 0
                && (stat(path, &mut st) != 0
                    || st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint))
            {
                if *path as libc::c_int == '/' as i32 || mode & 0o20 as libc::c_int == 0 {
                    return path;
                }
                dirs = getcwd(
                    cwd.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                );
                sep = 0 as libc::c_int;
            }
        }
        if dirs.is_null() {
            break;
        }
    }
    return 0 as *mut libc::c_char;
}
