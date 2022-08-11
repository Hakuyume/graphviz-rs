#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut gvplugin_dot_layout_LTX_library: gvplugin_library_t;
    static mut gvplugin_neato_layout_LTX_library: gvplugin_library_t;
    static mut gvplugin_gd_LTX_library: gvplugin_library_t;
    static mut gvplugin_pango_LTX_library: gvplugin_library_t;
    static mut gvplugin_webp_LTX_library: gvplugin_library_t;
    static mut gvplugin_core_LTX_library: gvplugin_library_t;
}
pub type api_t = libc::c_uint;
pub const API_loadimage: api_t = 4;
pub const API_device: api_t = 3;
pub const API_textlayout: api_t = 2;
pub const API_layout: api_t = 1;
pub const API_render: api_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lt_symlist_t {
    pub name: *const libc::c_char,
    pub address: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_library_t {
    pub packagename: *mut libc::c_char,
    pub apis: *mut gvplugin_api_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_api_t {
    pub api: api_t,
    pub types: *mut gvplugin_installed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_installed_t {
    pub id: libc::c_int,
    pub type_0: *const libc::c_char,
    pub quality: libc::c_int,
    pub engine: *mut libc::c_void,
    pub features: *mut libc::c_void,
}
#[no_mangle]
pub static mut lt_preloaded_symbols: [lt_symlist_t; 7] = unsafe {
    [
        {
            let mut init = lt_symlist_t {
                name: b"gvplugin_dot_layout_LTX_library\0" as *const u8
                    as *const libc::c_char,
                address: &gvplugin_dot_layout_LTX_library as *const gvplugin_library_t
                    as *mut gvplugin_library_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = lt_symlist_t {
                name: b"gvplugin_neato_layout_LTX_library\0" as *const u8
                    as *const libc::c_char,
                address: &gvplugin_neato_layout_LTX_library as *const gvplugin_library_t
                    as *mut gvplugin_library_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = lt_symlist_t {
                name: b"gvplugin_pango_LTX_library\0" as *const u8
                    as *const libc::c_char,
                address: &gvplugin_pango_LTX_library as *const gvplugin_library_t
                    as *mut gvplugin_library_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = lt_symlist_t {
                name: b"gvplugin_webp_LTX_library\0" as *const u8 as *const libc::c_char,
                address: &gvplugin_webp_LTX_library as *const gvplugin_library_t
                    as *mut gvplugin_library_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = lt_symlist_t {
                name: b"gvplugin_gd_LTX_library\0" as *const u8 as *const libc::c_char,
                address: &gvplugin_gd_LTX_library as *const gvplugin_library_t
                    as *mut gvplugin_library_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = lt_symlist_t {
                name: b"gvplugin_core_LTX_library\0" as *const u8 as *const libc::c_char,
                address: &gvplugin_core_LTX_library as *const gvplugin_library_t
                    as *mut gvplugin_library_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = lt_symlist_t {
                name: 0 as *const libc::c_char,
                address: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
    ]
};
