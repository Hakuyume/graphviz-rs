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
    pub type treegen_s;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    static mut optarg: *mut libc::c_char;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    static mut optopt: libc::c_int;
    fn makeBall(_: libc::c_int, _: libc::c_int, _: edgefn);
    fn makeCircle(_: libc::c_int, _: edgefn);
    fn makeComplete(_: libc::c_int, _: edgefn);
    fn makeCompleteB(_: libc::c_int, _: libc::c_int, _: edgefn);
    fn makePath(_: libc::c_int, _: edgefn);
    fn makeStar(_: libc::c_int, _: edgefn);
    fn makeWheel(_: libc::c_int, _: edgefn);
    fn makeTorus(_: libc::c_int, _: libc::c_int, _: edgefn);
    fn makeTwistedTorus(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: libc::c_int, _: edgefn);
    fn makeCylinder(_: libc::c_int, _: libc::c_int, _: edgefn);
    fn makeRandom(_: libc::c_int, _: libc::c_int, _: edgefn);
    fn makeSquareGrid(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: libc::c_int, _: edgefn);
    fn makeBinaryTree(_: libc::c_int, _: edgefn);
    fn makeSierpinski(_: libc::c_int, _: edgefn);
    fn makeTetrix(_: libc::c_int, _: edgefn);
    fn makeHypercube(_: libc::c_int, _: edgefn);
    fn makeTree(_: libc::c_int, _: libc::c_int, _: edgefn);
    fn makeTriMesh(_: libc::c_int, _: edgefn);
    fn makeMobius(_: libc::c_int, _: libc::c_int, _: edgefn);
    fn makeTreeGen(_: libc::c_int) -> *mut treegen_t;
    fn makeRandomTree(_: *mut treegen_t, _: edgefn);
    fn freeTreeGen(_: *mut treegen_t);
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
pub type edgefn = Option<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()>;
pub type treegen_t = treegen_s;
pub type GraphType = libc::c_uint;
pub const trimesh: GraphType = 17;
pub const wheel: GraphType = 16;
pub const star: GraphType = 15;
pub const hypercube: GraphType = 14;
pub const sierpinski: GraphType = 13;
pub const ball: GraphType = 12;
pub const randomt: GraphType = 11;
pub const randomg: GraphType = 10;
pub const mobius: GraphType = 9;
pub const cylinder: GraphType = 8;
pub const torus: GraphType = 7;
pub const tree: GraphType = 6;
pub const path: GraphType = 5;
pub const completeb: GraphType = 4;
pub const complete: GraphType = 3;
pub const circle: GraphType = 2;
pub const grid: GraphType = 1;
pub const unknown: GraphType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opts_t {
    pub graphSize1: libc::c_int,
    pub graphSize2: libc::c_int,
    pub cnt: libc::c_int,
    pub parm1: libc::c_int,
    pub parm2: libc::c_int,
    pub Verbose: libc::c_int,
    pub isPartial: libc::c_int,
    pub foldVal: libc::c_int,
    pub directed: libc::c_int,
    pub outfile: *mut FILE,
    pub pfx: *mut libc::c_char,
    pub name: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
static mut cmd: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn openFile(mut name: *const libc::c_char) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fprintf(
            stderr,
            b"%s: could not open file %s for writing\n\0" as *const u8 as *const libc::c_char,
            cmd,
            name,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return fp;
}
static mut Usage: *mut libc::c_char = b"Usage: %s [-dv?] [options]\n -c<n>         : cycle \n -C<x,y>       : cylinder \n -g[f]<h,w>    : grid (folded if f is used)\n -G[f]<h,w>    : partial grid (folded if f is used)\n -h<x>         : hypercube \n -k<x>         : complete \n -b<x,y>       : complete bipartite\n -B<x,y>       : ball\n -i<n>         : generate <n> random\n -m<x>         : triangular mesh\n -M<x,y>       : x by y Moebius strip\n -n<prefix>    : use <prefix> in node names (\"\")\n -N<name>      : use <name> for the graph (\"\")\n -o<outfile>   : put output in <outfile> (stdout)\n -p<x>         : path \n -r<x>,<n>     : random graph\n -R<n>         : random rooted tree on <n> vertices\n -s<x>         : star\n -S<x>         : 2D sierpinski\n -S<x>,<d>     : <d>D sierpinski (<d> = 2,3)\n -t<x>         : binary tree \n -t<x>,<n>     : n-ary tree \n -T<x,y>       : torus \n -T<x,y,t1,t2> : twisted torus \n -w<x>         : wheel\n -d            : directed graph\n -v            : verbose mode\n -?            : print usage\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn usage(mut v: libc::c_int) {
    fprintf(if v != 0 { stderr } else { stdout }, Usage, cmd);
    graphviz_exit(v);
}
unsafe extern "C" fn errexit(mut opt: libc::c_int) {
    fprintf(
        stderr,
        b"in flag -%c\n\0" as *const u8 as *const libc::c_char,
        opt as libc::c_char as libc::c_int,
    );
    usage(1 as libc::c_int);
}
unsafe extern "C" fn readPos(
    mut s: *mut libc::c_char,
    mut e: *mut *mut libc::c_char,
) -> libc::c_int {
    static mut MIN: libc::c_int = 1 as libc::c_int;
    let mut d: libc::c_long = strtol(s, e, 10 as libc::c_int);
    if s == *e || d > 2147483647 as libc::c_int as libc::c_long {
        fprintf(
            stderr,
            b"ill-formed integer \"%s\" \0" as *const u8 as *const libc::c_char,
            s,
        );
        return -(1 as libc::c_int);
    }
    if d < MIN as libc::c_long {
        fprintf(
            stderr,
            b"integer \"%s\" less than %d\0" as *const u8 as *const libc::c_char,
            s,
            MIN,
        );
        return -(1 as libc::c_int);
    }
    return d as libc::c_int;
}
unsafe extern "C" fn readOne(mut s: *mut libc::c_char, mut ip: *mut libc::c_int) -> libc::c_int {
    let mut d: libc::c_int = 0;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    d = readPos(s, &mut next);
    if d > 0 as libc::c_int {
        *ip = d;
        return 0 as libc::c_int;
    } else {
        return d;
    };
}
unsafe extern "C" fn setOne(mut s: *mut libc::c_char, mut opts_0: *mut opts_t) -> libc::c_int {
    let mut d: libc::c_int = 0;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    d = readPos(s, &mut next);
    if d > 0 as libc::c_int {
        (*opts_0).graphSize1 = d;
        return 0 as libc::c_int;
    } else {
        return d;
    };
}
unsafe extern "C" fn setTwo(mut s: *mut libc::c_char, mut opts_0: *mut opts_t) -> libc::c_int {
    let mut d: libc::c_int = 0;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    d = readPos(s, &mut next);
    if d < 0 as libc::c_int {
        return d;
    }
    (*opts_0).graphSize1 = d;
    if *next as libc::c_int != ',' as i32 {
        fprintf(
            stderr,
            b"ill-formed int pair \"%s\" \0" as *const u8 as *const libc::c_char,
            s,
        );
        return -(1 as libc::c_int);
    }
    s = next.offset(1 as libc::c_int as isize);
    d = readPos(s, &mut next);
    if d > 1 as libc::c_int {
        (*opts_0).graphSize2 = d;
        return 0 as libc::c_int;
    } else {
        return d;
    };
}
unsafe extern "C" fn setTwoTwoOpt(
    mut s: *mut libc::c_char,
    mut opts_0: *mut opts_t,
    mut dflt: libc::c_int,
) -> libc::c_int {
    let mut d: libc::c_int = 0;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    d = readPos(s, &mut next);
    if d < 0 as libc::c_int {
        return d;
    }
    (*opts_0).graphSize1 = d;
    if *next as libc::c_int != ',' as i32 {
        fprintf(
            stderr,
            b"ill-formed int pair \"%s\" \0" as *const u8 as *const libc::c_char,
            s,
        );
        return -(1 as libc::c_int);
    }
    s = next.offset(1 as libc::c_int as isize);
    d = readPos(s, &mut next);
    if d < 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*opts_0).graphSize2 = d;
    if *next as libc::c_int != ',' as i32 {
        let ref mut fresh0 = (*opts_0).parm2;
        *fresh0 = dflt;
        (*opts_0).parm1 = *fresh0;
        return 0 as libc::c_int;
    }
    s = next.offset(1 as libc::c_int as isize);
    d = readPos(s, &mut next);
    if d < 0 as libc::c_int {
        return d;
    }
    (*opts_0).parm1 = d;
    if *next as libc::c_int != ',' as i32 {
        (*opts_0).parm2 = dflt;
        return 0 as libc::c_int;
    }
    s = next.offset(1 as libc::c_int as isize);
    d = readPos(s, &mut next);
    if d < 0 as libc::c_int {
        return d;
    }
    (*opts_0).parm2 = d;
    return 0 as libc::c_int;
}
unsafe extern "C" fn setTwoOpt(
    mut s: *mut libc::c_char,
    mut opts_0: *mut opts_t,
    mut dflt: libc::c_int,
) -> libc::c_int {
    let mut d: libc::c_int = 0;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    d = readPos(s, &mut next);
    if d < 0 as libc::c_int {
        return d;
    }
    (*opts_0).graphSize1 = d;
    if *next as libc::c_int != ',' as i32 {
        (*opts_0).graphSize2 = dflt;
        return 0 as libc::c_int;
    }
    s = next.offset(1 as libc::c_int as isize);
    d = readPos(s, &mut next);
    if d > 1 as libc::c_int {
        (*opts_0).graphSize2 = d;
        return 0 as libc::c_int;
    } else {
        return d;
    };
}
unsafe extern "C" fn setFold(
    mut s: *mut libc::c_char,
    mut opts_0: *mut opts_t,
) -> *mut libc::c_char {
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    if *s as libc::c_int == 'f' as i32 {
        next = s.offset(1 as libc::c_int as isize);
        (*opts_0).foldVal = 1 as libc::c_int;
    } else {
        next = s;
    }
    return next;
}
static mut optList: *mut libc::c_char = b":i:M:m:n:N:c:C:dg:G:h:k:b:B:o:p:r:R:s:S:X:t:T:vw:\0"
    as *const u8 as *const libc::c_char
    as *mut libc::c_char;
unsafe extern "C" fn init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut opts_0: *mut opts_t,
) -> GraphType {
    let mut c: libc::c_int = 0;
    let mut graphType: GraphType = unknown;
    cmd = *argv.offset(0 as libc::c_int as isize);
    opterr = 0 as libc::c_int;
    loop {
        c = getopt(argc, argv as *const *mut libc::c_char, optList);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_70: u64;
        match c {
            99 => {
                graphType = circle;
                if setOne(optarg, opts_0) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            67 => {
                graphType = cylinder;
                if setTwo(optarg, opts_0) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            77 => {
                graphType = mobius;
                if setTwo(optarg, opts_0) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            100 => {
                (*opts_0).directed = 1 as libc::c_int;
                current_block_70 = 6838274324784804404;
            }
            71 => {
                (*opts_0).isPartial = 1 as libc::c_int;
                current_block_70 = 2181983174812170217;
            }
            103 => {
                current_block_70 = 2181983174812170217;
            }
            104 => {
                graphType = hypercube;
                if setOne(optarg, opts_0) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            107 => {
                graphType = complete;
                if setOne(optarg, opts_0) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            98 => {
                graphType = completeb;
                if setTwo(optarg, opts_0) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            66 => {
                graphType = ball;
                if setTwo(optarg, opts_0) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            109 => {
                graphType = trimesh;
                if setOne(optarg, opts_0) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            114 => {
                graphType = randomg;
                if setTwo(optarg, opts_0) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            82 => {
                graphType = randomt;
                if setOne(optarg, opts_0) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            110 => {
                let ref mut fresh1 = (*opts_0).pfx;
                *fresh1 = optarg;
                current_block_70 = 6838274324784804404;
            }
            78 => {
                let ref mut fresh2 = (*opts_0).name;
                *fresh2 = optarg;
                current_block_70 = 6838274324784804404;
            }
            111 => {
                let ref mut fresh3 = (*opts_0).outfile;
                *fresh3 = openFile(optarg);
                current_block_70 = 6838274324784804404;
            }
            112 => {
                graphType = path;
                if setOne(optarg, opts_0) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            83 => {
                graphType = sierpinski;
                if setTwoOpt(optarg, opts_0, 2 as libc::c_int) != 0 {
                    errexit(c);
                }
                if (*opts_0).graphSize2 > 3 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%dD Sierpinski not implemented - use 2 or 3 \0" as *const u8
                            as *const libc::c_char,
                        (*opts_0).graphSize2,
                    );
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            115 => {
                graphType = star;
                if setOne(optarg, opts_0) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            116 => {
                graphType = tree;
                if setTwoOpt(optarg, opts_0, 2 as libc::c_int) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            84 => {
                graphType = torus;
                if setTwoTwoOpt(optarg, opts_0, 0 as libc::c_int) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            105 => {
                if readOne(optarg, &mut (*opts_0).cnt) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            118 => {
                (*opts_0).Verbose = 1 as libc::c_int;
                current_block_70 = 6838274324784804404;
            }
            119 => {
                graphType = wheel;
                if setOne(optarg, opts_0) != 0 {
                    errexit(c);
                }
                current_block_70 = 6838274324784804404;
            }
            63 => {
                if optopt == '?' as i32 {
                    usage(0 as libc::c_int);
                } else {
                    fprintf(
                        stderr,
                        b"Unrecognized flag \"-%c\" - ignored\n\0" as *const u8
                            as *const libc::c_char,
                        optopt,
                    );
                }
                current_block_70 = 6838274324784804404;
            }
            _ => {
                fprintf(
                    stderr,
                    b"Unexpected error\n\0" as *const u8 as *const libc::c_char,
                );
                usage(1 as libc::c_int);
                current_block_70 = 6838274324784804404;
            }
        }
        match current_block_70 {
            2181983174812170217 => {
                graphType = grid;
                optarg = setFold(optarg, opts_0);
                if setTwo(optarg, opts_0) != 0 {
                    errexit(c);
                }
            }
            _ => {}
        }
    }
    argc -= optind;
    argv = argv.offset(optind as isize);
    if ((*opts_0).outfile).is_null() {
        let ref mut fresh4 = (*opts_0).outfile;
        *fresh4 = stdout;
    }
    if graphType as libc::c_uint == unknown as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"Graph type not set\n\0" as *const u8 as *const libc::c_char,
        );
        usage(1 as libc::c_int);
    }
    return graphType;
}
static mut opts: opts_t = opts_t {
    graphSize1: 0,
    graphSize2: 0,
    cnt: 0,
    parm1: 0,
    parm2: 0,
    Verbose: 0,
    isPartial: 0,
    foldVal: 0,
    directed: 0,
    outfile: 0 as *const FILE as *mut FILE,
    pfx: 0 as *const libc::c_char as *mut libc::c_char,
    name: 0 as *const libc::c_char as *mut libc::c_char,
};
unsafe extern "C" fn dirfn(mut t: libc::c_int, mut h: libc::c_int) {
    if h > 0 as libc::c_int {
        fprintf(
            opts.outfile,
            b"  %s%d -> %s%d\n\0" as *const u8 as *const libc::c_char,
            opts.pfx,
            t,
            opts.pfx,
            h,
        );
    } else {
        fprintf(
            opts.outfile,
            b"  %s%d\n\0" as *const u8 as *const libc::c_char,
            opts.pfx,
            t,
        );
    };
}
unsafe extern "C" fn undirfn(mut t: libc::c_int, mut h: libc::c_int) {
    if h > 0 as libc::c_int {
        fprintf(
            opts.outfile,
            b"  %s%d -- %s%d\n\0" as *const u8 as *const libc::c_char,
            opts.pfx,
            t,
            opts.pfx,
            h,
        );
    } else {
        fprintf(
            opts.outfile,
            b"  %s%d\n\0" as *const u8 as *const libc::c_char,
            opts.pfx,
            t,
        );
    };
}
unsafe extern "C" fn closeOpen() {
    if opts.directed != 0 {
        fprintf(
            opts.outfile,
            b"}\ndigraph {\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        fprintf(
            opts.outfile,
            b"}\ngraph {\n\0" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut graphType: GraphType = unknown;
    let mut ef: edgefn = None;
    opts.pfx = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    opts.name = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    opts.cnt = 1 as libc::c_int;
    graphType = init(argc, argv, &mut opts);
    if opts.directed != 0 {
        fprintf(
            opts.outfile,
            b"digraph %s{\n\0" as *const u8 as *const libc::c_char,
            opts.name,
        );
        ef = Some(dirfn as unsafe extern "C" fn(libc::c_int, libc::c_int) -> ());
    } else {
        fprintf(
            opts.outfile,
            b"graph %s{\n\0" as *const u8 as *const libc::c_char,
            opts.name,
        );
        ef = Some(undirfn as unsafe extern "C" fn(libc::c_int, libc::c_int) -> ());
    }
    match graphType as libc::c_uint {
        1 => {
            makeSquareGrid(
                opts.graphSize1,
                opts.graphSize2,
                opts.foldVal,
                opts.isPartial,
                ef,
            );
        }
        2 => {
            makeCircle(opts.graphSize1, ef);
        }
        5 => {
            makePath(opts.graphSize1, ef);
        }
        6 => {
            if opts.graphSize2 == 2 as libc::c_int {
                makeBinaryTree(opts.graphSize1, ef);
            } else {
                makeTree(opts.graphSize1, opts.graphSize2, ef);
            }
        }
        17 => {
            makeTriMesh(opts.graphSize1, ef);
        }
        12 => {
            makeBall(opts.graphSize1, opts.graphSize2, ef);
        }
        7 => {
            if opts.parm1 == 0 as libc::c_int && opts.parm2 == 0 as libc::c_int {
                makeTorus(opts.graphSize1, opts.graphSize2, ef);
            } else {
                makeTwistedTorus(opts.graphSize1, opts.graphSize2, opts.parm1, opts.parm2, ef);
            }
        }
        8 => {
            makeCylinder(opts.graphSize1, opts.graphSize2, ef);
        }
        9 => {
            makeMobius(opts.graphSize1, opts.graphSize2, ef);
        }
        13 => {
            if opts.graphSize2 == 2 as libc::c_int {
                makeSierpinski(opts.graphSize1, ef);
            } else {
                makeTetrix(opts.graphSize1, ef);
            }
        }
        3 => {
            makeComplete(opts.graphSize1, ef);
        }
        10 => {
            makeRandom(opts.graphSize1, opts.graphSize2, ef);
        }
        11 => {
            let mut i: libc::c_int = 0;
            let mut tg: *mut treegen_t = makeTreeGen(opts.graphSize1);
            i = 1 as libc::c_int;
            while i <= opts.cnt {
                makeRandomTree(tg, ef);
                if i != opts.cnt {
                    closeOpen();
                }
                i += 1;
            }
            freeTreeGen(tg);
            makeRandom(opts.graphSize1, opts.graphSize2, ef);
        }
        4 => {
            makeCompleteB(opts.graphSize1, opts.graphSize2, ef);
        }
        14 => {
            makeHypercube(opts.graphSize1, ef);
        }
        15 => {
            makeStar(opts.graphSize1, ef);
        }
        16 => {
            makeWheel(opts.graphSize1, ef);
        }
        _ => {}
    }
    fprintf(opts.outfile, b"}\n\0" as *const u8 as *const libc::c_char);
    graphviz_exit(0 as libc::c_int);
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
