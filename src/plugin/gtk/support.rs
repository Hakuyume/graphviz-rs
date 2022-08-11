#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _GData;
    pub type _PangoFontDescription;
    pub type _GdkRegion;
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_type_check_instance_cast(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> *mut GTypeInstance;
    fn g_type_check_instance_is_a(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> gboolean;
    fn g_object_get_data(object: *mut GObject, key: *const gchar) -> gpointer;
    fn gtk_menu_get_type() -> GType;
    fn gtk_menu_get_attach_widget(menu: *mut GtkMenu) -> *mut GtkWidget;
}
pub type guint8 = libc::c_uchar;
pub type guint16 = libc::c_ushort;
pub type guint32 = libc::c_uint;
pub type gint64 = libc::c_long;
pub type guint64 = libc::c_ulong;
pub type gsize = libc::c_ulong;
pub type gchar = libc::c_char;
pub type glong = libc::c_long;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type gulong = libc::c_ulong;
pub type guint = libc::c_uint;
pub type gfloat = libc::c_float;
pub type gdouble = libc::c_double;
pub type gpointer = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
pub type GQuark = guint32;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GLogLevelFlags = libc::c_int;
pub const G_LOG_LEVEL_MASK: GLogLevelFlags = -4;
pub const G_LOG_LEVEL_DEBUG: GLogLevelFlags = 128;
pub const G_LOG_LEVEL_INFO: GLogLevelFlags = 64;
pub const G_LOG_LEVEL_MESSAGE: GLogLevelFlags = 32;
pub const G_LOG_LEVEL_WARNING: GLogLevelFlags = 16;
pub const G_LOG_LEVEL_CRITICAL: GLogLevelFlags = 8;
pub const G_LOG_LEVEL_ERROR: GLogLevelFlags = 4;
pub const G_LOG_FLAG_FATAL: GLogLevelFlags = 2;
pub const G_LOG_FLAG_RECURSION: GLogLevelFlags = 1;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub v_int: gint,
    pub v_uint: guint,
    pub v_long: glong,
    pub v_ulong: gulong,
    pub v_int64: gint64,
    pub v_uint64: guint64,
    pub v_float: gfloat,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
}
pub type GValue = _GValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeClass {
    pub g_type: GType,
}
pub type GTypeClass = _GTypeClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _GClosure {
    #[bitfield(name = "ref_count", ty = "guint", bits = "0..=14")]
    #[bitfield(name = "meta_marshal_nouse", ty = "guint", bits = "15..=15")]
    #[bitfield(name = "n_guards", ty = "guint", bits = "16..=16")]
    #[bitfield(name = "n_fnotifiers", ty = "guint", bits = "17..=18")]
    #[bitfield(name = "n_inotifiers", ty = "guint", bits = "19..=26")]
    #[bitfield(name = "in_inotify", ty = "guint", bits = "27..=27")]
    #[bitfield(name = "floating", ty = "guint", bits = "28..=28")]
    #[bitfield(name = "derivative_flag", ty = "guint", bits = "29..=29")]
    #[bitfield(name = "in_marshal", ty = "guint", bits = "30..=30")]
    #[bitfield(name = "is_invalid", ty = "guint", bits = "31..=31")]
    pub ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub marshal: Option::<
        unsafe extern "C" fn(
            *mut GClosure,
            *mut GValue,
            guint,
            *const GValue,
            gpointer,
            gpointer,
        ) -> (),
    >,
    pub data: gpointer,
    pub notifiers: *mut GClosureNotifyData,
}
pub type GClosureNotifyData = _GClosureNotifyData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GClosureNotifyData {
    pub data: gpointer,
    pub notify: GClosureNotify,
}
pub type GClosureNotify = Option::<unsafe extern "C" fn(gpointer, *mut GClosure) -> ()>;
pub type GClosure = _GClosure;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
pub type GInitiallyUnowned = _GObject;
pub type PangoFontDescription = _PangoFontDescription;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkRectangle {
    pub x: gint,
    pub y: gint,
    pub width: gint,
    pub height: gint,
}
pub type GdkRectangle = _GdkRectangle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkColor {
    pub pixel: guint32,
    pub red: guint16,
    pub green: guint16,
    pub blue: guint16,
}
pub type GdkColor = _GdkColor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkColormap {
    pub parent_instance: GObject,
    pub size: gint,
    pub colors: *mut GdkColor,
    pub visual: *mut GdkVisual,
    pub windowing_data: gpointer,
}
pub type GdkVisual = _GdkVisual;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkVisual {
    pub parent_instance: GObject,
    pub type_0: GdkVisualType,
    pub depth: gint,
    pub byte_order: GdkByteOrder,
    pub colormap_size: gint,
    pub bits_per_rgb: gint,
    pub red_mask: guint32,
    pub red_shift: gint,
    pub red_prec: gint,
    pub green_mask: guint32,
    pub green_shift: gint,
    pub green_prec: gint,
    pub blue_mask: guint32,
    pub blue_shift: gint,
    pub blue_prec: gint,
}
pub type GdkByteOrder = libc::c_uint;
pub const GDK_MSB_FIRST: GdkByteOrder = 1;
pub const GDK_LSB_FIRST: GdkByteOrder = 0;
pub type GdkVisualType = libc::c_uint;
pub const GDK_VISUAL_DIRECT_COLOR: GdkVisualType = 5;
pub const GDK_VISUAL_TRUE_COLOR: GdkVisualType = 4;
pub const GDK_VISUAL_PSEUDO_COLOR: GdkVisualType = 3;
pub const GDK_VISUAL_STATIC_COLOR: GdkVisualType = 2;
pub const GDK_VISUAL_GRAYSCALE: GdkVisualType = 1;
pub const GDK_VISUAL_STATIC_GRAY: GdkVisualType = 0;
pub type GdkColormap = _GdkColormap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkFont {
    pub type_0: GdkFontType,
    pub ascent: gint,
    pub descent: gint,
}
pub type GdkFontType = libc::c_uint;
pub const GDK_FONT_FONTSET: GdkFontType = 1;
pub const GDK_FONT_FONT: GdkFontType = 0;
pub type GdkFont = _GdkFont;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkGC {
    pub parent_instance: GObject,
    pub clip_x_origin: gint,
    pub clip_y_origin: gint,
    pub ts_x_origin: gint,
    pub ts_y_origin: gint,
    pub colormap: *mut GdkColormap,
}
pub type GdkGC = _GdkGC;
pub type GdkRegion = _GdkRegion;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkDrawable {
    pub parent_instance: GObject,
}
pub type GdkPixmap = _GdkDrawable;
pub type GdkWindow = _GdkDrawable;
pub type GdkModifierType = libc::c_uint;
pub const GDK_MODIFIER_MASK: GdkModifierType = 1543512063;
pub const GDK_RELEASE_MASK: GdkModifierType = 1073741824;
pub const GDK_META_MASK: GdkModifierType = 268435456;
pub const GDK_HYPER_MASK: GdkModifierType = 134217728;
pub const GDK_SUPER_MASK: GdkModifierType = 67108864;
pub const GDK_BUTTON5_MASK: GdkModifierType = 4096;
pub const GDK_BUTTON4_MASK: GdkModifierType = 2048;
pub const GDK_BUTTON3_MASK: GdkModifierType = 1024;
pub const GDK_BUTTON2_MASK: GdkModifierType = 512;
pub const GDK_BUTTON1_MASK: GdkModifierType = 256;
pub const GDK_MOD5_MASK: GdkModifierType = 128;
pub const GDK_MOD4_MASK: GdkModifierType = 64;
pub const GDK_MOD3_MASK: GdkModifierType = 32;
pub const GDK_MOD2_MASK: GdkModifierType = 16;
pub const GDK_MOD1_MASK: GdkModifierType = 8;
pub const GDK_CONTROL_MASK: GdkModifierType = 4;
pub const GDK_LOCK_MASK: GdkModifierType = 2;
pub const GDK_SHIFT_MASK: GdkModifierType = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkAccelGroup {
    pub parent: GObject,
    pub lock_count: guint,
    pub modifier_mask: GdkModifierType,
    pub acceleratables: *mut GSList,
    pub n_accels: guint,
    pub priv_accels: *mut GtkAccelGroupEntry,
}
pub type GtkAccelGroupEntry = _GtkAccelGroupEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkAccelGroupEntry {
    pub key: GtkAccelKey,
    pub closure: *mut GClosure,
    pub accel_path_quark: GQuark,
}
pub type GtkAccelKey = _GtkAccelKey;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _GtkAccelKey {
    pub accel_key: guint,
    pub accel_mods: GdkModifierType,
    #[bitfield(name = "accel_flags", ty = "guint", bits = "0..=15")]
    pub accel_flags: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
pub type GtkAccelGroup = _GtkAccelGroup;
pub type GtkObject = _GtkObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkObject {
    pub parent_instance: GInitiallyUnowned,
    pub flags: guint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkAdjustment {
    pub parent_instance: GtkObject,
    pub lower: gdouble,
    pub upper: gdouble,
    pub value: gdouble,
    pub step_increment: gdouble,
    pub page_increment: gdouble,
    pub page_size: gdouble,
}
pub type GtkAdjustment = _GtkAdjustment;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkStyle {
    pub parent_instance: GObject,
    pub fg: [GdkColor; 5],
    pub bg: [GdkColor; 5],
    pub light: [GdkColor; 5],
    pub dark: [GdkColor; 5],
    pub mid: [GdkColor; 5],
    pub text: [GdkColor; 5],
    pub base: [GdkColor; 5],
    pub text_aa: [GdkColor; 5],
    pub black: GdkColor,
    pub white: GdkColor,
    pub font_desc: *mut PangoFontDescription,
    pub xthickness: gint,
    pub ythickness: gint,
    pub fg_gc: [*mut GdkGC; 5],
    pub bg_gc: [*mut GdkGC; 5],
    pub light_gc: [*mut GdkGC; 5],
    pub dark_gc: [*mut GdkGC; 5],
    pub mid_gc: [*mut GdkGC; 5],
    pub text_gc: [*mut GdkGC; 5],
    pub base_gc: [*mut GdkGC; 5],
    pub text_aa_gc: [*mut GdkGC; 5],
    pub black_gc: *mut GdkGC,
    pub white_gc: *mut GdkGC,
    pub bg_pixmap: [*mut GdkPixmap; 5],
    pub attach_count: gint,
    pub depth: gint,
    pub colormap: *mut GdkColormap,
    pub private_font: *mut GdkFont,
    pub private_font_desc: *mut PangoFontDescription,
    pub rc_style: *mut GtkRcStyle,
    pub styles: *mut GSList,
    pub property_cache: *mut GArray,
    pub icon_factories: *mut GSList,
}
pub type GtkRcStyle = _GtkRcStyle;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _GtkRcStyle {
    pub parent_instance: GObject,
    pub name: *mut gchar,
    pub bg_pixmap_name: [*mut gchar; 5],
    pub font_desc: *mut PangoFontDescription,
    pub color_flags: [GtkRcFlags; 5],
    pub fg: [GdkColor; 5],
    pub bg: [GdkColor; 5],
    pub text: [GdkColor; 5],
    pub base: [GdkColor; 5],
    pub xthickness: gint,
    pub ythickness: gint,
    pub rc_properties: *mut GArray,
    pub rc_style_lists: *mut GSList,
    pub icon_factories: *mut GSList,
    #[bitfield(name = "engine_specified", ty = "guint", bits = "0..=0")]
    pub engine_specified: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type GtkRcFlags = libc::c_uint;
pub const GTK_RC_BASE: GtkRcFlags = 8;
pub const GTK_RC_TEXT: GtkRcFlags = 4;
pub const GTK_RC_BG: GtkRcFlags = 2;
pub const GTK_RC_FG: GtkRcFlags = 1;
pub type GtkStyle = _GtkStyle;
pub type GtkWidget = _GtkWidget;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkWidget {
    pub object: GtkObject,
    pub private_flags: guint16,
    pub state: guint8,
    pub saved_state: guint8,
    pub name: *mut gchar,
    pub style: *mut GtkStyle,
    pub requisition: GtkRequisition,
    pub allocation: GtkAllocation,
    pub window: *mut GdkWindow,
    pub parent: *mut GtkWidget,
}
pub type GtkAllocation = GdkRectangle;
pub type GtkRequisition = _GtkRequisition;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkRequisition {
    pub width: gint,
    pub height: gint,
}
pub type GtkContainer = _GtkContainer;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _GtkContainer {
    pub widget: GtkWidget,
    pub focus_child: *mut GtkWidget,
    #[bitfield(name = "border_width", ty = "guint", bits = "0..=15")]
    #[bitfield(name = "need_resize", ty = "guint", bits = "16..=16")]
    #[bitfield(name = "resize_mode", ty = "guint", bits = "17..=18")]
    #[bitfield(name = "reallocate_redraws", ty = "guint", bits = "19..=19")]
    #[bitfield(name = "has_focus_chain", ty = "guint", bits = "20..=20")]
    pub border_width_need_resize_resize_mode_reallocate_redraws_has_focus_chain: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
}
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _GtkMenuShell {
    pub container: GtkContainer,
    pub children: *mut GList,
    pub active_menu_item: *mut GtkWidget,
    pub parent_menu_shell: *mut GtkWidget,
    pub button: guint,
    pub activate_time: guint32,
    #[bitfield(name = "active", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "have_grab", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "have_xgrab", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "ignore_leave", ty = "guint", bits = "3..=3")]
    #[bitfield(name = "menu_flag", ty = "guint", bits = "4..=4")]
    #[bitfield(name = "ignore_enter", ty = "guint", bits = "5..=5")]
    #[bitfield(name = "keyboard_mode", ty = "guint", bits = "6..=6")]
    pub active_have_grab_have_xgrab_ignore_leave_menu_flag_ignore_enter_keyboard_mode: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type GtkMenuShell = _GtkMenuShell;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _GtkMenu {
    pub menu_shell: GtkMenuShell,
    pub parent_menu_item: *mut GtkWidget,
    pub old_active_menu_item: *mut GtkWidget,
    pub accel_group: *mut GtkAccelGroup,
    pub accel_path: *mut gchar,
    pub position_func: GtkMenuPositionFunc,
    pub position_func_data: gpointer,
    pub toggle_size: guint,
    pub toplevel: *mut GtkWidget,
    pub tearoff_window: *mut GtkWidget,
    pub tearoff_hbox: *mut GtkWidget,
    pub tearoff_scrollbar: *mut GtkWidget,
    pub tearoff_adjustment: *mut GtkAdjustment,
    pub view_window: *mut GdkWindow,
    pub bin_window: *mut GdkWindow,
    pub scroll_offset: gint,
    pub saved_scroll_offset: gint,
    pub scroll_step: gint,
    pub timeout_id: guint,
    pub navigation_region: *mut GdkRegion,
    pub navigation_timeout: guint,
    #[bitfield(name = "needs_destruction_ref_count", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "torn_off", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "tearoff_active", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "scroll_fast", ty = "guint", bits = "3..=3")]
    #[bitfield(name = "upper_arrow_visible", ty = "guint", bits = "4..=4")]
    #[bitfield(name = "lower_arrow_visible", ty = "guint", bits = "5..=5")]
    #[bitfield(name = "upper_arrow_prelight", ty = "guint", bits = "6..=6")]
    #[bitfield(name = "lower_arrow_prelight", ty = "guint", bits = "7..=7")]
    pub needs_destruction_ref_count_torn_off_tearoff_active_scroll_fast_upper_arrow_visible_lower_arrow_visible_upper_arrow_prelight_lower_arrow_prelight: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type GtkMenuPositionFunc = Option::<
    unsafe extern "C" fn(
        *mut GtkMenu,
        *mut gint,
        *mut gint,
        *mut gboolean,
        gpointer,
    ) -> (),
>;
pub type GtkMenu = _GtkMenu;
#[no_mangle]
pub unsafe extern "C" fn lookup_widget(
    mut widget: *mut GtkWidget,
    mut widget_name: *const gchar,
) -> *mut GtkWidget {
    let mut parent: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut found_widget: *mut GtkWidget = 0 as *mut GtkWidget;
    loop {
        if ({
            let mut __inst: *mut GTypeInstance = widget as *mut GTypeInstance;
            let mut __t: GType = gtk_menu_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as libc::c_int;
            } else if !((*__inst).g_class).is_null()
                    && (*(*__inst).g_class).g_type == __t
                {
                __r = (0 as libc::c_int == 0) as libc::c_int;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            parent = gtk_menu_get_attach_widget(
                g_type_check_instance_cast(
                    widget as *mut GTypeInstance,
                    gtk_menu_get_type(),
                ) as *mut libc::c_void as *mut GtkMenu,
            );
        } else {
            parent = (*widget).parent;
        }
        if parent.is_null() {
            parent = g_object_get_data(
                g_type_check_instance_cast(
                    widget as *mut GTypeInstance,
                    ((20 as libc::c_int) << 2 as libc::c_int) as GType,
                ) as *mut libc::c_void as *mut GObject,
                b"GladeParentKey\0" as *const u8 as *const libc::c_char,
            ) as *mut GtkWidget;
        }
        if parent.is_null() {
            break;
        }
        widget = parent;
    }
    found_widget = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        widget_name,
    ) as *mut GtkWidget;
    if found_widget.is_null() {
        g_log(
            0 as *mut gchar,
            G_LOG_LEVEL_WARNING,
            b"Widget not found: %s\0" as *const u8 as *const libc::c_char,
            widget_name,
        );
    }
    return found_widget;
}
