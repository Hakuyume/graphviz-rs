#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut gvdevice_dot_types: [gvplugin_installed_t; 0];
    static mut gvdevice_fig_types: [gvplugin_installed_t; 0];
    static mut gvdevice_map_types: [gvplugin_installed_t; 0];
    static mut gvdevice_mp_types: [gvplugin_installed_t; 0];
    static mut gvdevice_ps_types: [gvplugin_installed_t; 0];
    static mut gvdevice_svg_types: [gvplugin_installed_t; 0];
    static mut gvdevice_json_types: [gvplugin_installed_t; 0];
    static mut gvdevice_tk_types: [gvplugin_installed_t; 0];
    static mut gvdevice_vml_types: [gvplugin_installed_t; 0];
    static mut gvdevice_pic_types: [gvplugin_installed_t; 0];
    static mut gvdevice_pov_types: [gvplugin_installed_t; 0];
    static mut gvrender_dot_types: [gvplugin_installed_t; 0];
    static mut gvrender_fig_types: [gvplugin_installed_t; 0];
    static mut gvrender_map_types: [gvplugin_installed_t; 0];
    static mut gvrender_mp_types: [gvplugin_installed_t; 0];
    static mut gvrender_ps_types: [gvplugin_installed_t; 0];
    static mut gvrender_svg_types: [gvplugin_installed_t; 0];
    static mut gvrender_json_types: [gvplugin_installed_t; 0];
    static mut gvrender_tk_types: [gvplugin_installed_t; 0];
    static mut gvrender_vml_types: [gvplugin_installed_t; 0];
    static mut gvrender_pic_types: [gvplugin_installed_t; 0];
    static mut gvrender_pov_types: [gvplugin_installed_t; 0];
    static mut gvloadimage_core_types: [gvplugin_installed_t; 0];
}
pub type api_t = libc::c_uint;
pub const API_loadimage: api_t = 4;
pub const API_device: api_t = 3;
pub const API_textlayout: api_t = 2;
pub const API_layout: api_t = 1;
pub const API_render: api_t = 0;
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
pub struct gvplugin_api_t {
    pub api: api_t,
    pub types: *mut gvplugin_installed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_library_t {
    pub packagename: *mut libc::c_char,
    pub apis: *mut gvplugin_api_t,
}
static mut apis: [gvplugin_api_t; 24] = unsafe {
    [
        {
            let mut init = gvplugin_api_t {
                api: API_device,
                types: gvdevice_dot_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_device,
                types: gvdevice_fig_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_device,
                types: gvdevice_map_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_device,
                types: gvdevice_mp_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_device,
                types: gvdevice_ps_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_device,
                types: gvdevice_svg_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_device,
                types: gvdevice_json_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_device,
                types: gvdevice_tk_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_device,
                types: gvdevice_vml_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_device,
                types: gvdevice_pic_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_device,
                types: gvdevice_pov_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_render,
                types: gvrender_dot_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_render,
                types: gvrender_fig_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_render,
                types: gvrender_map_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_render,
                types: gvrender_mp_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_render,
                types: gvrender_ps_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_render,
                types: gvrender_svg_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_render,
                types: gvrender_json_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_render,
                types: gvrender_tk_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_render,
                types: gvrender_vml_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_render,
                types: gvrender_pic_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_render,
                types: gvrender_pov_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_loadimage,
                types: gvloadimage_core_types.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = gvplugin_api_t {
                api: API_render,
                types: 0 as *const gvplugin_installed_t as *mut gvplugin_installed_t,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut gvplugin_core_LTX_library: gvplugin_library_t = unsafe {
    {
        let mut init = gvplugin_library_t {
            packagename: b"core\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            apis: apis.as_ptr() as *mut _,
        };
        init
    }
};
