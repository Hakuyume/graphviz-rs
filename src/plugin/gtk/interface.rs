#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _GData;
    pub type _cairo_font_options;
    pub type _PangoFontDescription;
    pub type _GdkAtom;
    pub type _GdkRegion;
    pub type _GtkWindowGeometryInfo;
    pub type _GtkTreeViewPrivate;
    pub type _GtkPanedPrivate;
    pub type _GtkToolItemPrivate;
    fn g_type_check_instance_cast(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> *mut GTypeInstance;
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_object_set_data(object: *mut GObject, key: *const gchar, data: gpointer);
    fn g_object_set_data_full(
        object: *mut GObject,
        key: *const gchar,
        data: gpointer,
        destroy: GDestroyNotify,
    );
    fn gtk_toolbar_set_style(toolbar: *mut GtkToolbar, style: GtkToolbarStyle);
    fn gtk_toolbar_new() -> *mut GtkWidget;
    fn gtk_toolbar_get_type() -> GType;
    fn gtk_accel_group_new() -> *mut GtkAccelGroup;
    fn gtk_object_get_type() -> GType;
    fn gtk_widget_ref(widget: *mut GtkWidget) -> *mut GtkWidget;
    fn gtk_widget_unref(widget: *mut GtkWidget);
    fn gtk_widget_show(widget: *mut GtkWidget);
    fn gtk_widget_set_sensitive(widget: *mut GtkWidget, sensitive: gboolean);
    fn gtk_widget_set_size_request(widget: *mut GtkWidget, width: gint, height: gint);
    fn gtk_widget_set_events(widget: *mut GtkWidget, events: gint);
    fn gtk_container_get_type() -> GType;
    fn gtk_container_add(container: *mut GtkContainer, widget: *mut GtkWidget);
    fn gtk_window_get_type() -> GType;
    fn gtk_window_new(type_0: GtkWindowType) -> *mut GtkWidget;
    fn gtk_window_set_title(window: *mut GtkWindow, title: *const gchar);
    fn gtk_window_add_accel_group(
        window: *mut GtkWindow,
        accel_group: *mut GtkAccelGroup,
    );
    fn gtk_menu_new() -> *mut GtkWidget;
    fn gtk_label_new(str: *const gchar) -> *mut GtkWidget;
    fn gtk_box_get_type() -> GType;
    fn gtk_box_pack_start(
        box_0: *mut GtkBox,
        child: *mut GtkWidget,
        expand: gboolean,
        fill: gboolean,
        padding: guint,
    );
    fn gtk_box_pack_end(
        box_0: *mut GtkBox,
        child: *mut GtkWidget,
        expand: gboolean,
        fill: gboolean,
        padding: guint,
    );
    fn gtk_button_get_type() -> GType;
    fn gtk_button_new_with_mnemonic(label: *const gchar) -> *mut GtkWidget;
    fn gtk_button_set_focus_on_click(button: *mut GtkButton, focus_on_click: gboolean);
    fn gtk_menu_item_get_type() -> GType;
    fn gtk_menu_item_new_with_mnemonic(label: *const gchar) -> *mut GtkWidget;
    fn gtk_menu_item_set_submenu(menu_item: *mut GtkMenuItem, submenu: *mut GtkWidget);
    fn gtk_vbox_new(homogeneous: gboolean, spacing: gint) -> *mut GtkWidget;
    fn gtk_tree_view_get_type() -> GType;
    fn gtk_tree_view_new() -> *mut GtkWidget;
    fn gtk_tree_view_set_headers_visible(
        tree_view: *mut GtkTreeView,
        headers_visible: gboolean,
    );
    fn gtk_tree_view_set_rules_hint(tree_view: *mut GtkTreeView, setting: gboolean);
    fn gtk_drawing_area_new() -> *mut GtkWidget;
    fn gtk_hbox_new(homogeneous: gboolean, spacing: gint) -> *mut GtkWidget;
    fn gtk_paned_get_type() -> GType;
    fn gtk_paned_pack1(
        paned: *mut GtkPaned,
        child: *mut GtkWidget,
        resize: gboolean,
        shrink: gboolean,
    );
    fn gtk_paned_pack2(
        paned: *mut GtkPaned,
        child: *mut GtkWidget,
        resize: gboolean,
        shrink: gboolean,
    );
    fn gtk_paned_set_position(paned: *mut GtkPaned, position: gint);
    fn gtk_hpaned_new() -> *mut GtkWidget;
    fn gtk_image_menu_item_new_from_stock(
        stock_id: *const gchar,
        accel_group: *mut GtkAccelGroup,
    ) -> *mut GtkWidget;
    fn gtk_menu_bar_new() -> *mut GtkWidget;
    fn gtk_tool_item_get_type() -> GType;
    fn gtk_tool_item_new() -> *mut GtkToolItem;
    fn gtk_tool_item_set_expand(tool_item: *mut GtkToolItem, expand: gboolean);
    fn gtk_scrolled_window_get_type() -> GType;
    fn gtk_scrolled_window_new(
        hadjustment: *mut GtkAdjustment,
        vadjustment: *mut GtkAdjustment,
    ) -> *mut GtkWidget;
    fn gtk_scrolled_window_set_policy(
        scrolled_window: *mut GtkScrolledWindow,
        hscrollbar_policy: GtkPolicyType,
        vscrollbar_policy: GtkPolicyType,
    );
    fn gtk_scrolled_window_set_shadow_type(
        scrolled_window: *mut GtkScrolledWindow,
        type_0: GtkShadowType,
    );
    fn gtk_separator_menu_item_new() -> *mut GtkWidget;
    fn on_new1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer);
    fn on_open1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer);
    fn on_save1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer);
    fn on_save_as1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer);
    fn on_quit1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer);
    fn on_cut1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer);
    fn on_copy1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer);
    fn on_paste1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer);
    fn on_delete1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer);
    fn on_about1_activate(menuitem: *mut GtkMenuItem, user_data: gpointer);
    fn on_drawingarea1_expose_event(
        widget: *mut GtkWidget,
        event: *mut GdkEventExpose,
        user_data: gpointer,
    ) -> gboolean;
    fn on_drawingarea1_motion_notify_event(
        widget: *mut GtkWidget,
        event: *mut GdkEventMotion,
        user_data: gpointer,
    ) -> gboolean;
    fn on_drawingarea2_motion_notify_event(
        widget: *mut GtkWidget,
        event: *mut GdkEventMotion,
        user_data: gpointer,
    ) -> gboolean;
    fn on_drawingarea2_expose_event(
        widget: *mut GtkWidget,
        event: *mut GdkEventExpose,
        user_data: gpointer,
    ) -> gboolean;
    fn on_window1_delete_event(
        widget: *mut GtkWidget,
        event: *mut GdkEvent,
        user_data: gpointer,
    ) -> gboolean;
    fn on_drawingarea1_configure_event(
        widget: *mut GtkWidget,
        event: *mut GdkEventConfigure,
        user_data: gpointer,
    ) -> gboolean;
    fn on_drawingarea1_button_press_event(
        widget: *mut GtkWidget,
        event: *mut GdkEventButton,
        user_data: gpointer,
    ) -> gboolean;
    fn on_drawingarea1_button_release_event(
        widget: *mut GtkWidget,
        event: *mut GdkEventButton,
        user_data: gpointer,
    ) -> gboolean;
    fn on_drawingarea1_scroll_event(
        widget: *mut GtkWidget,
        event: *mut GdkEvent,
        user_data: gpointer,
    ) -> gboolean;
    fn on_button1_button_press_event(
        widget: *mut GtkWidget,
        event: *mut GdkEventButton,
        user_data: gpointer,
    ) -> gboolean;
}
pub type gint8 = libc::c_schar;
pub type guint8 = libc::c_uchar;
pub type gint16 = libc::c_short;
pub type guint16 = libc::c_ushort;
pub type guint32 = libc::c_uint;
pub type gint64 = libc::c_long;
pub type guint64 = libc::c_ulong;
pub type gsize = libc::c_ulong;
pub type gchar = libc::c_char;
pub type gshort = libc::c_short;
pub type glong = libc::c_long;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type gushort = libc::c_ushort;
pub type gulong = libc::c_ulong;
pub type guint = libc::c_uint;
pub type gfloat = libc::c_float;
pub type gdouble = libc::c_double;
pub type gpointer = *mut libc::c_void;
pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTimeVal {
    pub tv_sec: glong,
    pub tv_usec: glong,
}
pub type GTimeVal = _GTimeVal;
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
#[derive(Copy, Clone, BitfieldStruct)]
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
pub type GCallback = Option::<unsafe extern "C" fn() -> ()>;
pub type GConnectFlags = libc::c_uint;
pub const G_CONNECT_SWAPPED: GConnectFlags = 2;
pub const G_CONNECT_AFTER: GConnectFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
pub type GInitiallyUnowned = _GObject;
pub type cairo_font_options_t = _cairo_font_options;
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
pub type GdkAtom = *mut _GdkAtom;
pub type GdkNativeWindow = guint32;
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
pub type GdkCursorType = libc::c_int;
pub const GDK_CURSOR_IS_PIXMAP: GdkCursorType = -1;
pub const GDK_BLANK_CURSOR: GdkCursorType = -2;
pub const GDK_LAST_CURSOR: GdkCursorType = 153;
pub const GDK_XTERM: GdkCursorType = 152;
pub const GDK_WATCH: GdkCursorType = 150;
pub const GDK_UR_ANGLE: GdkCursorType = 148;
pub const GDK_UMBRELLA: GdkCursorType = 146;
pub const GDK_UL_ANGLE: GdkCursorType = 144;
pub const GDK_TREK: GdkCursorType = 142;
pub const GDK_TOP_TEE: GdkCursorType = 140;
pub const GDK_TOP_SIDE: GdkCursorType = 138;
pub const GDK_TOP_RIGHT_CORNER: GdkCursorType = 136;
pub const GDK_TOP_LEFT_CORNER: GdkCursorType = 134;
pub const GDK_TOP_LEFT_ARROW: GdkCursorType = 132;
pub const GDK_TCROSS: GdkCursorType = 130;
pub const GDK_TARGET: GdkCursorType = 128;
pub const GDK_STAR: GdkCursorType = 126;
pub const GDK_SPRAYCAN: GdkCursorType = 124;
pub const GDK_SPIDER: GdkCursorType = 122;
pub const GDK_SIZING: GdkCursorType = 120;
pub const GDK_SHUTTLE: GdkCursorType = 118;
pub const GDK_SB_V_DOUBLE_ARROW: GdkCursorType = 116;
pub const GDK_SB_UP_ARROW: GdkCursorType = 114;
pub const GDK_SB_RIGHT_ARROW: GdkCursorType = 112;
pub const GDK_SB_LEFT_ARROW: GdkCursorType = 110;
pub const GDK_SB_H_DOUBLE_ARROW: GdkCursorType = 108;
pub const GDK_SB_DOWN_ARROW: GdkCursorType = 106;
pub const GDK_SAILBOAT: GdkCursorType = 104;
pub const GDK_RTL_LOGO: GdkCursorType = 102;
pub const GDK_RIGHTBUTTON: GdkCursorType = 100;
pub const GDK_RIGHT_TEE: GdkCursorType = 98;
pub const GDK_RIGHT_SIDE: GdkCursorType = 96;
pub const GDK_RIGHT_PTR: GdkCursorType = 94;
pub const GDK_QUESTION_ARROW: GdkCursorType = 92;
pub const GDK_PLUS: GdkCursorType = 90;
pub const GDK_PIRATE: GdkCursorType = 88;
pub const GDK_PENCIL: GdkCursorType = 86;
pub const GDK_MOUSE: GdkCursorType = 84;
pub const GDK_MIDDLEBUTTON: GdkCursorType = 82;
pub const GDK_MAN: GdkCursorType = 80;
pub const GDK_LR_ANGLE: GdkCursorType = 78;
pub const GDK_LL_ANGLE: GdkCursorType = 76;
pub const GDK_LEFTBUTTON: GdkCursorType = 74;
pub const GDK_LEFT_TEE: GdkCursorType = 72;
pub const GDK_LEFT_SIDE: GdkCursorType = 70;
pub const GDK_LEFT_PTR: GdkCursorType = 68;
pub const GDK_IRON_CROSS: GdkCursorType = 66;
pub const GDK_ICON: GdkCursorType = 64;
pub const GDK_HEART: GdkCursorType = 62;
pub const GDK_HAND2: GdkCursorType = 60;
pub const GDK_HAND1: GdkCursorType = 58;
pub const GDK_GUMBY: GdkCursorType = 56;
pub const GDK_GOBBLER: GdkCursorType = 54;
pub const GDK_FLEUR: GdkCursorType = 52;
pub const GDK_EXCHANGE: GdkCursorType = 50;
pub const GDK_DRAPED_BOX: GdkCursorType = 48;
pub const GDK_DRAFT_SMALL: GdkCursorType = 46;
pub const GDK_DRAFT_LARGE: GdkCursorType = 44;
pub const GDK_DOUBLE_ARROW: GdkCursorType = 42;
pub const GDK_DOTBOX: GdkCursorType = 40;
pub const GDK_DOT: GdkCursorType = 38;
pub const GDK_DIAMOND_CROSS: GdkCursorType = 36;
pub const GDK_CROSSHAIR: GdkCursorType = 34;
pub const GDK_CROSS_REVERSE: GdkCursorType = 32;
pub const GDK_CROSS: GdkCursorType = 30;
pub const GDK_COFFEE_MUG: GdkCursorType = 28;
pub const GDK_CLOCK: GdkCursorType = 26;
pub const GDK_CIRCLE: GdkCursorType = 24;
pub const GDK_CENTER_PTR: GdkCursorType = 22;
pub const GDK_BOX_SPIRAL: GdkCursorType = 20;
pub const GDK_BOTTOM_TEE: GdkCursorType = 18;
pub const GDK_BOTTOM_SIDE: GdkCursorType = 16;
pub const GDK_BOTTOM_RIGHT_CORNER: GdkCursorType = 14;
pub const GDK_BOTTOM_LEFT_CORNER: GdkCursorType = 12;
pub const GDK_BOGOSITY: GdkCursorType = 10;
pub const GDK_BOAT: GdkCursorType = 8;
pub const GDK_BASED_ARROW_UP: GdkCursorType = 6;
pub const GDK_BASED_ARROW_DOWN: GdkCursorType = 4;
pub const GDK_ARROW: GdkCursorType = 2;
pub const GDK_X_CURSOR: GdkCursorType = 0;
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
pub type GdkScreen = _GdkScreen;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GdkScreen {
    pub parent_instance: GObject,
    #[bitfield(name = "closed", ty = "guint", bits = "0..=0")]
    pub closed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub normal_gcs: [*mut GdkGC; 32],
    pub exposure_gcs: [*mut GdkGC; 32],
    pub subwindow_gcs: [*mut GdkGC; 32],
    pub font_options: *mut cairo_font_options_t,
    pub resolution: libc::c_double,
}
pub type GdkDevice = _GdkDevice;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkDevice {
    pub parent_instance: GObject,
    pub name: *mut gchar,
    pub source: GdkInputSource,
    pub mode: GdkInputMode,
    pub has_cursor: gboolean,
    pub num_axes: gint,
    pub axes: *mut GdkDeviceAxis,
    pub num_keys: gint,
    pub keys: *mut GdkDeviceKey,
}
pub type GdkDeviceKey = _GdkDeviceKey;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkDeviceKey {
    pub keyval: guint,
    pub modifiers: GdkModifierType,
}
pub type GdkDeviceAxis = _GdkDeviceAxis;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkDeviceAxis {
    pub use_0: GdkAxisUse,
    pub min: gdouble,
    pub max: gdouble,
}
pub type GdkAxisUse = libc::c_uint;
pub const GDK_AXIS_LAST: GdkAxisUse = 7;
pub const GDK_AXIS_WHEEL: GdkAxisUse = 6;
pub const GDK_AXIS_YTILT: GdkAxisUse = 5;
pub const GDK_AXIS_XTILT: GdkAxisUse = 4;
pub const GDK_AXIS_PRESSURE: GdkAxisUse = 3;
pub const GDK_AXIS_Y: GdkAxisUse = 2;
pub const GDK_AXIS_X: GdkAxisUse = 1;
pub const GDK_AXIS_IGNORE: GdkAxisUse = 0;
pub type GdkInputMode = libc::c_uint;
pub const GDK_MODE_WINDOW: GdkInputMode = 2;
pub const GDK_MODE_SCREEN: GdkInputMode = 1;
pub const GDK_MODE_DISABLED: GdkInputMode = 0;
pub type GdkInputSource = libc::c_uint;
pub const GDK_SOURCE_CURSOR: GdkInputSource = 3;
pub const GDK_SOURCE_ERASER: GdkInputSource = 2;
pub const GDK_SOURCE_PEN: GdkInputSource = 1;
pub const GDK_SOURCE_MOUSE: GdkInputSource = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkDragContext {
    pub parent_instance: GObject,
    pub protocol: GdkDragProtocol,
    pub is_source: gboolean,
    pub source_window: *mut GdkWindow,
    pub dest_window: *mut GdkWindow,
    pub targets: *mut GList,
    pub actions: GdkDragAction,
    pub suggested_action: GdkDragAction,
    pub action: GdkDragAction,
    pub start_time: guint32,
    pub windowing_data: gpointer,
}
pub type GdkDragAction = libc::c_uint;
pub const GDK_ACTION_ASK: GdkDragAction = 32;
pub const GDK_ACTION_PRIVATE: GdkDragAction = 16;
pub const GDK_ACTION_LINK: GdkDragAction = 8;
pub const GDK_ACTION_MOVE: GdkDragAction = 4;
pub const GDK_ACTION_COPY: GdkDragAction = 2;
pub const GDK_ACTION_DEFAULT: GdkDragAction = 1;
pub type GdkDragProtocol = libc::c_uint;
pub const GDK_DRAG_PROTO_LOCAL: GdkDragProtocol = 6;
pub const GDK_DRAG_PROTO_OLE2: GdkDragProtocol = 5;
pub const GDK_DRAG_PROTO_WIN32_DROPFILES: GdkDragProtocol = 4;
pub const GDK_DRAG_PROTO_NONE: GdkDragProtocol = 3;
pub const GDK_DRAG_PROTO_ROOTWIN: GdkDragProtocol = 2;
pub const GDK_DRAG_PROTO_XDND: GdkDragProtocol = 1;
pub const GDK_DRAG_PROTO_MOTIF: GdkDragProtocol = 0;
pub type GdkDragContext = _GdkDragContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventAny {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
}
pub type GdkEventType = libc::c_int;
pub const GDK_EVENT_LAST: GdkEventType = 37;
pub const GDK_DAMAGE: GdkEventType = 36;
pub const GDK_GRAB_BROKEN: GdkEventType = 35;
pub const GDK_OWNER_CHANGE: GdkEventType = 34;
pub const GDK_SETTING: GdkEventType = 33;
pub const GDK_WINDOW_STATE: GdkEventType = 32;
pub const GDK_SCROLL: GdkEventType = 31;
pub const GDK_NO_EXPOSE: GdkEventType = 30;
pub const GDK_VISIBILITY_NOTIFY: GdkEventType = 29;
pub const GDK_CLIENT_EVENT: GdkEventType = 28;
pub const GDK_DROP_FINISHED: GdkEventType = 27;
pub const GDK_DROP_START: GdkEventType = 26;
pub const GDK_DRAG_STATUS: GdkEventType = 25;
pub const GDK_DRAG_MOTION: GdkEventType = 24;
pub const GDK_DRAG_LEAVE: GdkEventType = 23;
pub const GDK_DRAG_ENTER: GdkEventType = 22;
pub const GDK_PROXIMITY_OUT: GdkEventType = 21;
pub const GDK_PROXIMITY_IN: GdkEventType = 20;
pub const GDK_SELECTION_NOTIFY: GdkEventType = 19;
pub const GDK_SELECTION_REQUEST: GdkEventType = 18;
pub const GDK_SELECTION_CLEAR: GdkEventType = 17;
pub const GDK_PROPERTY_NOTIFY: GdkEventType = 16;
pub const GDK_UNMAP: GdkEventType = 15;
pub const GDK_MAP: GdkEventType = 14;
pub const GDK_CONFIGURE: GdkEventType = 13;
pub const GDK_FOCUS_CHANGE: GdkEventType = 12;
pub const GDK_LEAVE_NOTIFY: GdkEventType = 11;
pub const GDK_ENTER_NOTIFY: GdkEventType = 10;
pub const GDK_KEY_RELEASE: GdkEventType = 9;
pub const GDK_KEY_PRESS: GdkEventType = 8;
pub const GDK_BUTTON_RELEASE: GdkEventType = 7;
pub const GDK_3BUTTON_PRESS: GdkEventType = 6;
pub const GDK_2BUTTON_PRESS: GdkEventType = 5;
pub const GDK_BUTTON_PRESS: GdkEventType = 4;
pub const GDK_MOTION_NOTIFY: GdkEventType = 3;
pub const GDK_EXPOSE: GdkEventType = 2;
pub const GDK_DESTROY: GdkEventType = 1;
pub const GDK_DELETE: GdkEventType = 0;
pub const GDK_NOTHING: GdkEventType = -1;
pub type GdkEventAny = _GdkEventAny;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventExpose {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub area: GdkRectangle,
    pub region: *mut GdkRegion,
    pub count: gint,
}
pub type GdkEventExpose = _GdkEventExpose;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventNoExpose {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
}
pub type GdkEventNoExpose = _GdkEventNoExpose;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventVisibility {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub state: GdkVisibilityState,
}
pub type GdkVisibilityState = libc::c_uint;
pub const GDK_VISIBILITY_FULLY_OBSCURED: GdkVisibilityState = 2;
pub const GDK_VISIBILITY_PARTIAL: GdkVisibilityState = 1;
pub const GDK_VISIBILITY_UNOBSCURED: GdkVisibilityState = 0;
pub type GdkEventVisibility = _GdkEventVisibility;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventMotion {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub time: guint32,
    pub x: gdouble,
    pub y: gdouble,
    pub axes: *mut gdouble,
    pub state: guint,
    pub is_hint: gint16,
    pub device: *mut GdkDevice,
    pub x_root: gdouble,
    pub y_root: gdouble,
}
pub type GdkEventMotion = _GdkEventMotion;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventButton {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub time: guint32,
    pub x: gdouble,
    pub y: gdouble,
    pub axes: *mut gdouble,
    pub state: guint,
    pub button: guint,
    pub device: *mut GdkDevice,
    pub x_root: gdouble,
    pub y_root: gdouble,
}
pub type GdkEventButton = _GdkEventButton;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventScroll {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub time: guint32,
    pub x: gdouble,
    pub y: gdouble,
    pub state: guint,
    pub direction: GdkScrollDirection,
    pub device: *mut GdkDevice,
    pub x_root: gdouble,
    pub y_root: gdouble,
}
pub type GdkScrollDirection = libc::c_uint;
pub const GDK_SCROLL_RIGHT: GdkScrollDirection = 3;
pub const GDK_SCROLL_LEFT: GdkScrollDirection = 2;
pub const GDK_SCROLL_DOWN: GdkScrollDirection = 1;
pub const GDK_SCROLL_UP: GdkScrollDirection = 0;
pub type GdkEventScroll = _GdkEventScroll;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GdkEventKey {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub time: guint32,
    pub state: guint,
    pub keyval: guint,
    pub length: gint,
    pub string: *mut gchar,
    pub hardware_keycode: guint16,
    pub group: guint8,
    #[bitfield(name = "is_modifier", ty = "guint", bits = "0..=0")]
    pub is_modifier: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type GdkEventKey = _GdkEventKey;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventFocus {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub in_0: gint16,
}
pub type GdkEventFocus = _GdkEventFocus;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventCrossing {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub subwindow: *mut GdkWindow,
    pub time: guint32,
    pub x: gdouble,
    pub y: gdouble,
    pub x_root: gdouble,
    pub y_root: gdouble,
    pub mode: GdkCrossingMode,
    pub detail: GdkNotifyType,
    pub focus: gboolean,
    pub state: guint,
}
pub type GdkNotifyType = libc::c_uint;
pub const GDK_NOTIFY_UNKNOWN: GdkNotifyType = 5;
pub const GDK_NOTIFY_NONLINEAR_VIRTUAL: GdkNotifyType = 4;
pub const GDK_NOTIFY_NONLINEAR: GdkNotifyType = 3;
pub const GDK_NOTIFY_INFERIOR: GdkNotifyType = 2;
pub const GDK_NOTIFY_VIRTUAL: GdkNotifyType = 1;
pub const GDK_NOTIFY_ANCESTOR: GdkNotifyType = 0;
pub type GdkCrossingMode = libc::c_uint;
pub const GDK_CROSSING_STATE_CHANGED: GdkCrossingMode = 5;
pub const GDK_CROSSING_GTK_UNGRAB: GdkCrossingMode = 4;
pub const GDK_CROSSING_GTK_GRAB: GdkCrossingMode = 3;
pub const GDK_CROSSING_UNGRAB: GdkCrossingMode = 2;
pub const GDK_CROSSING_GRAB: GdkCrossingMode = 1;
pub const GDK_CROSSING_NORMAL: GdkCrossingMode = 0;
pub type GdkEventCrossing = _GdkEventCrossing;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventConfigure {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub x: gint,
    pub y: gint,
    pub width: gint,
    pub height: gint,
}
pub type GdkEventConfigure = _GdkEventConfigure;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventProperty {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub atom: GdkAtom,
    pub time: guint32,
    pub state: guint,
}
pub type GdkEventProperty = _GdkEventProperty;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventSelection {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub selection: GdkAtom,
    pub target: GdkAtom,
    pub property: GdkAtom,
    pub time: guint32,
    pub requestor: GdkNativeWindow,
}
pub type GdkEventSelection = _GdkEventSelection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventOwnerChange {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub owner: GdkNativeWindow,
    pub reason: GdkOwnerChange,
    pub selection: GdkAtom,
    pub time: guint32,
    pub selection_time: guint32,
}
pub type GdkOwnerChange = libc::c_uint;
pub const GDK_OWNER_CHANGE_CLOSE: GdkOwnerChange = 2;
pub const GDK_OWNER_CHANGE_DESTROY: GdkOwnerChange = 1;
pub const GDK_OWNER_CHANGE_NEW_OWNER: GdkOwnerChange = 0;
pub type GdkEventOwnerChange = _GdkEventOwnerChange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventProximity {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub time: guint32,
    pub device: *mut GdkDevice,
}
pub type GdkEventProximity = _GdkEventProximity;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventClient {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub message_type: GdkAtom,
    pub data_format: gushort,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}
pub type GdkEventClient = _GdkEventClient;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventDND {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub context: *mut GdkDragContext,
    pub time: guint32,
    pub x_root: gshort,
    pub y_root: gshort,
}
pub type GdkEventDND = _GdkEventDND;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventWindowState {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub changed_mask: GdkWindowState,
    pub new_window_state: GdkWindowState,
}
pub type GdkWindowState = libc::c_uint;
pub const GDK_WINDOW_STATE_BELOW: GdkWindowState = 64;
pub const GDK_WINDOW_STATE_ABOVE: GdkWindowState = 32;
pub const GDK_WINDOW_STATE_FULLSCREEN: GdkWindowState = 16;
pub const GDK_WINDOW_STATE_STICKY: GdkWindowState = 8;
pub const GDK_WINDOW_STATE_MAXIMIZED: GdkWindowState = 4;
pub const GDK_WINDOW_STATE_ICONIFIED: GdkWindowState = 2;
pub const GDK_WINDOW_STATE_WITHDRAWN: GdkWindowState = 1;
pub type GdkEventWindowState = _GdkEventWindowState;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventSetting {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub action: GdkSettingAction,
    pub name: *mut libc::c_char,
}
pub type GdkSettingAction = libc::c_uint;
pub const GDK_SETTING_ACTION_DELETED: GdkSettingAction = 2;
pub const GDK_SETTING_ACTION_CHANGED: GdkSettingAction = 1;
pub const GDK_SETTING_ACTION_NEW: GdkSettingAction = 0;
pub type GdkEventSetting = _GdkEventSetting;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkEventGrabBroken {
    pub type_0: GdkEventType,
    pub window: *mut GdkWindow,
    pub send_event: gint8,
    pub keyboard: gboolean,
    pub implicit: gboolean,
    pub grab_window: *mut GdkWindow,
}
pub type GdkEventGrabBroken = _GdkEventGrabBroken;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GdkEvent {
    pub type_0: GdkEventType,
    pub any: GdkEventAny,
    pub expose: GdkEventExpose,
    pub no_expose: GdkEventNoExpose,
    pub visibility: GdkEventVisibility,
    pub motion: GdkEventMotion,
    pub button: GdkEventButton,
    pub scroll: GdkEventScroll,
    pub key: GdkEventKey,
    pub crossing: GdkEventCrossing,
    pub focus_change: GdkEventFocus,
    pub configure: GdkEventConfigure,
    pub property: GdkEventProperty,
    pub selection: GdkEventSelection,
    pub owner_change: GdkEventOwnerChange,
    pub proximity: GdkEventProximity,
    pub client: GdkEventClient,
    pub dnd: GdkEventDND,
    pub window_state: GdkEventWindowState,
    pub setting: GdkEventSetting,
    pub grab_broken: GdkEventGrabBroken,
}
pub type GdkEvent = _GdkEvent;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const GDK_ALL_EVENTS_MASK: C2RustUnnamed_1 = 4194302;
pub const GDK_SCROLL_MASK: C2RustUnnamed_1 = 2097152;
pub const GDK_SUBSTRUCTURE_MASK: C2RustUnnamed_1 = 1048576;
pub const GDK_PROXIMITY_OUT_MASK: C2RustUnnamed_1 = 524288;
pub const GDK_PROXIMITY_IN_MASK: C2RustUnnamed_1 = 262144;
pub const GDK_VISIBILITY_NOTIFY_MASK: C2RustUnnamed_1 = 131072;
pub const GDK_PROPERTY_CHANGE_MASK: C2RustUnnamed_1 = 65536;
pub const GDK_STRUCTURE_MASK: C2RustUnnamed_1 = 32768;
pub const GDK_FOCUS_CHANGE_MASK: C2RustUnnamed_1 = 16384;
pub const GDK_LEAVE_NOTIFY_MASK: C2RustUnnamed_1 = 8192;
pub const GDK_ENTER_NOTIFY_MASK: C2RustUnnamed_1 = 4096;
pub const GDK_KEY_RELEASE_MASK: C2RustUnnamed_1 = 2048;
pub const GDK_KEY_PRESS_MASK: C2RustUnnamed_1 = 1024;
pub const GDK_BUTTON_RELEASE_MASK: C2RustUnnamed_1 = 512;
pub const GDK_BUTTON_PRESS_MASK: C2RustUnnamed_1 = 256;
pub const GDK_BUTTON3_MOTION_MASK: C2RustUnnamed_1 = 128;
pub const GDK_BUTTON2_MOTION_MASK: C2RustUnnamed_1 = 64;
pub const GDK_BUTTON1_MOTION_MASK: C2RustUnnamed_1 = 32;
pub const GDK_BUTTON_MOTION_MASK: C2RustUnnamed_1 = 16;
pub const GDK_POINTER_MOTION_HINT_MASK: C2RustUnnamed_1 = 8;
pub const GDK_POINTER_MOTION_MASK: C2RustUnnamed_1 = 4;
pub const GDK_EXPOSURE_MASK: C2RustUnnamed_1 = 2;
pub type GtkIconSize = libc::c_uint;
pub const GTK_ICON_SIZE_DIALOG: GtkIconSize = 6;
pub const GTK_ICON_SIZE_DND: GtkIconSize = 5;
pub const GTK_ICON_SIZE_BUTTON: GtkIconSize = 4;
pub const GTK_ICON_SIZE_LARGE_TOOLBAR: GtkIconSize = 3;
pub const GTK_ICON_SIZE_SMALL_TOOLBAR: GtkIconSize = 2;
pub const GTK_ICON_SIZE_MENU: GtkIconSize = 1;
pub const GTK_ICON_SIZE_INVALID: GtkIconSize = 0;
pub type GtkOrientation = libc::c_uint;
pub const GTK_ORIENTATION_VERTICAL: GtkOrientation = 1;
pub const GTK_ORIENTATION_HORIZONTAL: GtkOrientation = 0;
pub type GtkPolicyType = libc::c_uint;
pub const GTK_POLICY_NEVER: GtkPolicyType = 2;
pub const GTK_POLICY_AUTOMATIC: GtkPolicyType = 1;
pub const GTK_POLICY_ALWAYS: GtkPolicyType = 0;
pub type GtkShadowType = libc::c_uint;
pub const GTK_SHADOW_ETCHED_OUT: GtkShadowType = 4;
pub const GTK_SHADOW_ETCHED_IN: GtkShadowType = 3;
pub const GTK_SHADOW_OUT: GtkShadowType = 2;
pub const GTK_SHADOW_IN: GtkShadowType = 1;
pub const GTK_SHADOW_NONE: GtkShadowType = 0;
pub type GtkToolbarStyle = libc::c_uint;
pub const GTK_TOOLBAR_BOTH_HORIZ: GtkToolbarStyle = 3;
pub const GTK_TOOLBAR_BOTH: GtkToolbarStyle = 2;
pub const GTK_TOOLBAR_TEXT: GtkToolbarStyle = 1;
pub const GTK_TOOLBAR_ICONS: GtkToolbarStyle = 0;
pub type GtkWindowType = libc::c_uint;
pub const GTK_WINDOW_POPUP: GtkWindowType = 1;
pub const GTK_WINDOW_TOPLEVEL: GtkWindowType = 0;
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
#[derive(Copy, Clone, BitfieldStruct)]
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
#[derive(Copy, Clone, BitfieldStruct)]
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GtkWindow {
    pub bin: GtkBin,
    pub title: *mut gchar,
    pub wmclass_name: *mut gchar,
    pub wmclass_class: *mut gchar,
    pub wm_role: *mut gchar,
    pub focus_widget: *mut GtkWidget,
    pub default_widget: *mut GtkWidget,
    pub transient_parent: *mut GtkWindow,
    pub geometry_info: *mut GtkWindowGeometryInfo,
    pub frame: *mut GdkWindow,
    pub group: *mut GtkWindowGroup,
    pub configure_request_count: guint16,
    #[bitfield(name = "allow_shrink", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "allow_grow", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "configure_notify_received", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "need_default_position", ty = "guint", bits = "3..=3")]
    #[bitfield(name = "need_default_size", ty = "guint", bits = "4..=4")]
    #[bitfield(name = "position", ty = "guint", bits = "5..=7")]
    #[bitfield(name = "type_0", ty = "guint", bits = "8..=11")]
    #[bitfield(name = "has_user_ref_count", ty = "guint", bits = "12..=12")]
    #[bitfield(name = "has_focus", ty = "guint", bits = "13..=13")]
    #[bitfield(name = "modal", ty = "guint", bits = "14..=14")]
    #[bitfield(name = "destroy_with_parent", ty = "guint", bits = "15..=15")]
    #[bitfield(name = "has_frame", ty = "guint", bits = "16..=16")]
    #[bitfield(name = "iconify_initially", ty = "guint", bits = "17..=17")]
    #[bitfield(name = "stick_initially", ty = "guint", bits = "18..=18")]
    #[bitfield(name = "maximize_initially", ty = "guint", bits = "19..=19")]
    #[bitfield(name = "decorated", ty = "guint", bits = "20..=20")]
    #[bitfield(name = "type_hint", ty = "guint", bits = "21..=23")]
    #[bitfield(name = "gravity", ty = "guint", bits = "24..=28")]
    #[bitfield(name = "is_active", ty = "guint", bits = "29..=29")]
    #[bitfield(name = "has_toplevel_focus", ty = "guint", bits = "30..=30")]
    pub allow_shrink_allow_grow_configure_notify_received_need_default_position_need_default_size_position_type_0_has_user_ref_count_has_focus_modal_destroy_with_parent_has_frame_iconify_initially_stick_initially_maximize_initially_decorated_type_hint_gravity_is_active_has_toplevel_focus: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub frame_left: guint,
    pub frame_top: guint,
    pub frame_right: guint,
    pub frame_bottom: guint,
    pub keys_changed_handler: guint,
    pub mnemonic_modifier: GdkModifierType,
    pub screen: *mut GdkScreen,
}
pub type GtkWindowGroup = _GtkWindowGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkWindowGroup {
    pub parent_instance: GObject,
    pub grabs: *mut GSList,
}
pub type GtkWindowGeometryInfo = _GtkWindowGeometryInfo;
pub type GtkWindow = _GtkWindow;
pub type GtkBin = _GtkBin;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkBin {
    pub container: GtkContainer,
    pub child: *mut GtkWidget,
}
pub type GtkContainer = _GtkContainer;
#[derive(Copy, Clone, BitfieldStruct)]
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GtkBox {
    pub container: GtkContainer,
    pub children: *mut GList,
    pub spacing: gint16,
    #[bitfield(name = "homogeneous", ty = "guint", bits = "0..=0")]
    pub homogeneous: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
}
pub type GtkBox = _GtkBox;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GtkButton {
    pub bin: GtkBin,
    pub event_window: *mut GdkWindow,
    pub label_text: *mut gchar,
    pub activate_timeout: guint,
    #[bitfield(name = "constructed", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "in_button", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "button_down", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "relief", ty = "guint", bits = "3..=4")]
    #[bitfield(name = "use_underline", ty = "guint", bits = "5..=5")]
    #[bitfield(name = "use_stock", ty = "guint", bits = "6..=6")]
    #[bitfield(name = "depressed", ty = "guint", bits = "7..=7")]
    #[bitfield(name = "depress_on_activate", ty = "guint", bits = "8..=8")]
    #[bitfield(name = "focus_on_click", ty = "guint", bits = "9..=9")]
    pub constructed_in_button_button_down_relief_use_underline_use_stock_depressed_depress_on_activate_focus_on_click: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
pub type GtkButton = _GtkButton;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkItem {
    pub bin: GtkBin,
}
pub type GtkItem = _GtkItem;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GtkMenuItem {
    pub item: GtkItem,
    pub submenu: *mut GtkWidget,
    pub event_window: *mut GdkWindow,
    pub toggle_size: guint16,
    pub accelerator_width: guint16,
    pub accel_path: *mut gchar,
    #[bitfield(name = "show_submenu_indicator", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "submenu_placement", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "submenu_direction", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "right_justify", ty = "guint", bits = "3..=3")]
    #[bitfield(name = "timer_from_keypress", ty = "guint", bits = "4..=4")]
    #[bitfield(name = "from_menubar", ty = "guint", bits = "5..=5")]
    pub show_submenu_indicator_submenu_placement_submenu_direction_right_justify_timer_from_keypress_from_menubar: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub timer: guint,
}
pub type GtkMenuItem = _GtkMenuItem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkTreeView {
    pub parent: GtkContainer,
    pub priv_0: *mut GtkTreeViewPrivate,
}
pub type GtkTreeViewPrivate = _GtkTreeViewPrivate;
pub type GtkTreeView = _GtkTreeView;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GtkPaned {
    pub container: GtkContainer,
    pub child1: *mut GtkWidget,
    pub child2: *mut GtkWidget,
    pub handle: *mut GdkWindow,
    pub xor_gc: *mut GdkGC,
    pub cursor_type: GdkCursorType,
    pub handle_pos: GdkRectangle,
    pub child1_size: gint,
    pub last_allocation: gint,
    pub min_position: gint,
    pub max_position: gint,
    #[bitfield(name = "position_set", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "in_drag", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "child1_shrink", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "child1_resize", ty = "guint", bits = "3..=3")]
    #[bitfield(name = "child2_shrink", ty = "guint", bits = "4..=4")]
    #[bitfield(name = "child2_resize", ty = "guint", bits = "5..=5")]
    #[bitfield(name = "orientation", ty = "guint", bits = "6..=6")]
    #[bitfield(name = "in_recursion", ty = "guint", bits = "7..=7")]
    #[bitfield(name = "handle_prelit", ty = "guint", bits = "8..=8")]
    pub position_set_in_drag_child1_shrink_child1_resize_child2_shrink_child2_resize_orientation_in_recursion_handle_prelit: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub last_child1_focus: *mut GtkWidget,
    pub last_child2_focus: *mut GtkWidget,
    pub priv_0: *mut GtkPanedPrivate,
    pub drag_pos: gint,
    pub original_position: gint,
}
pub type GtkPanedPrivate = _GtkPanedPrivate;
pub type GtkPaned = _GtkPaned;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GtkTooltips {
    pub parent_instance: GtkObject,
    pub tip_window: *mut GtkWidget,
    pub tip_label: *mut GtkWidget,
    pub active_tips_data: *mut GtkTooltipsData,
    pub tips_data_list: *mut GList,
    #[bitfield(name = "delay", ty = "guint", bits = "0..=29")]
    #[bitfield(name = "enabled", ty = "guint", bits = "30..=30")]
    #[bitfield(name = "have_grab", ty = "guint", bits = "31..=31")]
    #[bitfield(name = "use_sticky_delay", ty = "guint", bits = "32..=32")]
    pub delay_enabled_have_grab_use_sticky_delay: [u8; 5],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub timer_tag: gint,
    pub last_popdown: GTimeVal,
}
pub type GtkTooltipsData = _GtkTooltipsData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkTooltipsData {
    pub tooltips: *mut GtkTooltips,
    pub widget: *mut GtkWidget,
    pub tip_text: *mut gchar,
    pub tip_private: *mut gchar,
}
pub type GtkTooltips = _GtkTooltips;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkToolItem {
    pub parent: GtkBin,
    pub priv_0: *mut GtkToolItemPrivate,
}
pub type GtkToolItemPrivate = _GtkToolItemPrivate;
pub type GtkToolItem = _GtkToolItem;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GtkScrolledWindow {
    pub container: GtkBin,
    pub hscrollbar: *mut GtkWidget,
    pub vscrollbar: *mut GtkWidget,
    #[bitfield(name = "hscrollbar_policy", ty = "guint", bits = "0..=1")]
    #[bitfield(name = "vscrollbar_policy", ty = "guint", bits = "2..=3")]
    #[bitfield(name = "hscrollbar_visible", ty = "guint", bits = "4..=4")]
    #[bitfield(name = "vscrollbar_visible", ty = "guint", bits = "5..=5")]
    #[bitfield(name = "window_placement", ty = "guint", bits = "6..=7")]
    #[bitfield(name = "focus_out", ty = "guint", bits = "8..=8")]
    pub hscrollbar_policy_vscrollbar_policy_hscrollbar_visible_vscrollbar_visible_window_placement_focus_out: [u8; 2],
    pub shadow_type: guint16,
}
pub type GtkScrolledWindow = _GtkScrolledWindow;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GtkToolbar {
    pub container: GtkContainer,
    pub num_children: gint,
    pub children: *mut GList,
    pub orientation: GtkOrientation,
    pub style: GtkToolbarStyle,
    pub icon_size: GtkIconSize,
    pub tooltips: *mut GtkTooltips,
    pub button_maxw: gint,
    pub button_maxh: gint,
    pub _gtk_reserved1: guint,
    pub _gtk_reserved2: guint,
    #[bitfield(name = "style_set", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "icon_size_set", ty = "guint", bits = "1..=1")]
    pub style_set_icon_size_set: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type GtkToolbar = _GtkToolbar;
#[no_mangle]
pub unsafe extern "C" fn create_window1() -> *mut GtkWidget {
    let mut window1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut vbox1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut menubar1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut menuitem4: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut menu4: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut new1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut open1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut save1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut save_as1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut separatormenuitem1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut quit1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut menuitem5: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut menu5: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut cut1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut copy1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut paste1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut delete1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut menuitem6: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut menu6: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut menuitem7: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut menu7: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut about1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut hpaned1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut vbox2: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut hbox2: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut drawingarea2: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut scrolledwindow3: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut treeview1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut toolbar1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut toolitem1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut label1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut toolitem2: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut label2: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut toolitem3: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut button1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut scrolledwindow4: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut treeview2: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut drawingarea1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut accel_group: *mut GtkAccelGroup = 0 as *mut GtkAccelGroup;
    accel_group = gtk_accel_group_new();
    window1 = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(
        g_type_check_instance_cast(window1 as *mut GTypeInstance, gtk_window_get_type())
            as *mut libc::c_void as *mut GtkWindow,
        b"DotEdit\0" as *const u8 as *const libc::c_char,
    );
    vbox1 = gtk_vbox_new(0 as libc::c_int, 0 as libc::c_int);
    gtk_widget_show(vbox1);
    gtk_container_add(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut libc::c_void as *mut GtkContainer,
        vbox1,
    );
    menubar1 = gtk_menu_bar_new();
    gtk_widget_show(menubar1);
    gtk_box_pack_start(
        g_type_check_instance_cast(vbox1 as *mut GTypeInstance, gtk_box_get_type())
            as *mut libc::c_void as *mut GtkBox,
        menubar1,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int as guint,
    );
    menuitem4 = gtk_menu_item_new_with_mnemonic(
        b"_File\0" as *const u8 as *const libc::c_char,
    );
    gtk_widget_show(menuitem4);
    gtk_container_add(
        g_type_check_instance_cast(
            menubar1 as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut libc::c_void as *mut GtkContainer,
        menuitem4,
    );
    menu4 = gtk_menu_new();
    gtk_menu_item_set_submenu(
        g_type_check_instance_cast(
            menuitem4 as *mut GTypeInstance,
            gtk_menu_item_get_type(),
        ) as *mut libc::c_void as *mut GtkMenuItem,
        menu4,
    );
    new1 = gtk_image_menu_item_new_from_stock(
        b"gtk-new\0" as *const u8 as *const libc::c_char,
        accel_group,
    );
    gtk_widget_show(new1);
    gtk_container_add(
        g_type_check_instance_cast(menu4 as *mut GTypeInstance, gtk_container_get_type())
            as *mut libc::c_void as *mut GtkContainer,
        new1,
    );
    open1 = gtk_image_menu_item_new_from_stock(
        b"gtk-open\0" as *const u8 as *const libc::c_char,
        accel_group,
    );
    gtk_widget_show(open1);
    gtk_container_add(
        g_type_check_instance_cast(menu4 as *mut GTypeInstance, gtk_container_get_type())
            as *mut libc::c_void as *mut GtkContainer,
        open1,
    );
    save1 = gtk_image_menu_item_new_from_stock(
        b"gtk-save\0" as *const u8 as *const libc::c_char,
        accel_group,
    );
    gtk_widget_show(save1);
    gtk_container_add(
        g_type_check_instance_cast(menu4 as *mut GTypeInstance, gtk_container_get_type())
            as *mut libc::c_void as *mut GtkContainer,
        save1,
    );
    save_as1 = gtk_image_menu_item_new_from_stock(
        b"gtk-save-as\0" as *const u8 as *const libc::c_char,
        accel_group,
    );
    gtk_widget_show(save_as1);
    gtk_container_add(
        g_type_check_instance_cast(menu4 as *mut GTypeInstance, gtk_container_get_type())
            as *mut libc::c_void as *mut GtkContainer,
        save_as1,
    );
    separatormenuitem1 = gtk_separator_menu_item_new();
    gtk_widget_show(separatormenuitem1);
    gtk_container_add(
        g_type_check_instance_cast(menu4 as *mut GTypeInstance, gtk_container_get_type())
            as *mut libc::c_void as *mut GtkContainer,
        separatormenuitem1,
    );
    gtk_widget_set_sensitive(separatormenuitem1, 0 as libc::c_int);
    quit1 = gtk_image_menu_item_new_from_stock(
        b"gtk-quit\0" as *const u8 as *const libc::c_char,
        accel_group,
    );
    gtk_widget_show(quit1);
    gtk_container_add(
        g_type_check_instance_cast(menu4 as *mut GTypeInstance, gtk_container_get_type())
            as *mut libc::c_void as *mut GtkContainer,
        quit1,
    );
    menuitem5 = gtk_menu_item_new_with_mnemonic(
        b"_Edit\0" as *const u8 as *const libc::c_char,
    );
    gtk_widget_show(menuitem5);
    gtk_container_add(
        g_type_check_instance_cast(
            menubar1 as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut libc::c_void as *mut GtkContainer,
        menuitem5,
    );
    menu5 = gtk_menu_new();
    gtk_menu_item_set_submenu(
        g_type_check_instance_cast(
            menuitem5 as *mut GTypeInstance,
            gtk_menu_item_get_type(),
        ) as *mut libc::c_void as *mut GtkMenuItem,
        menu5,
    );
    cut1 = gtk_image_menu_item_new_from_stock(
        b"gtk-cut\0" as *const u8 as *const libc::c_char,
        accel_group,
    );
    gtk_widget_show(cut1);
    gtk_container_add(
        g_type_check_instance_cast(menu5 as *mut GTypeInstance, gtk_container_get_type())
            as *mut libc::c_void as *mut GtkContainer,
        cut1,
    );
    copy1 = gtk_image_menu_item_new_from_stock(
        b"gtk-copy\0" as *const u8 as *const libc::c_char,
        accel_group,
    );
    gtk_widget_show(copy1);
    gtk_container_add(
        g_type_check_instance_cast(menu5 as *mut GTypeInstance, gtk_container_get_type())
            as *mut libc::c_void as *mut GtkContainer,
        copy1,
    );
    paste1 = gtk_image_menu_item_new_from_stock(
        b"gtk-paste\0" as *const u8 as *const libc::c_char,
        accel_group,
    );
    gtk_widget_show(paste1);
    gtk_container_add(
        g_type_check_instance_cast(menu5 as *mut GTypeInstance, gtk_container_get_type())
            as *mut libc::c_void as *mut GtkContainer,
        paste1,
    );
    delete1 = gtk_image_menu_item_new_from_stock(
        b"gtk-delete\0" as *const u8 as *const libc::c_char,
        accel_group,
    );
    gtk_widget_show(delete1);
    gtk_container_add(
        g_type_check_instance_cast(menu5 as *mut GTypeInstance, gtk_container_get_type())
            as *mut libc::c_void as *mut GtkContainer,
        delete1,
    );
    menuitem6 = gtk_menu_item_new_with_mnemonic(
        b"_View\0" as *const u8 as *const libc::c_char,
    );
    gtk_widget_show(menuitem6);
    gtk_container_add(
        g_type_check_instance_cast(
            menubar1 as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut libc::c_void as *mut GtkContainer,
        menuitem6,
    );
    menu6 = gtk_menu_new();
    gtk_menu_item_set_submenu(
        g_type_check_instance_cast(
            menuitem6 as *mut GTypeInstance,
            gtk_menu_item_get_type(),
        ) as *mut libc::c_void as *mut GtkMenuItem,
        menu6,
    );
    menuitem7 = gtk_menu_item_new_with_mnemonic(
        b"_Help\0" as *const u8 as *const libc::c_char,
    );
    gtk_widget_show(menuitem7);
    gtk_container_add(
        g_type_check_instance_cast(
            menubar1 as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut libc::c_void as *mut GtkContainer,
        menuitem7,
    );
    menu7 = gtk_menu_new();
    gtk_menu_item_set_submenu(
        g_type_check_instance_cast(
            menuitem7 as *mut GTypeInstance,
            gtk_menu_item_get_type(),
        ) as *mut libc::c_void as *mut GtkMenuItem,
        menu7,
    );
    about1 = gtk_menu_item_new_with_mnemonic(
        b"_About\0" as *const u8 as *const libc::c_char,
    );
    gtk_widget_show(about1);
    gtk_container_add(
        g_type_check_instance_cast(menu7 as *mut GTypeInstance, gtk_container_get_type())
            as *mut libc::c_void as *mut GtkContainer,
        about1,
    );
    hpaned1 = gtk_hpaned_new();
    gtk_widget_show(hpaned1);
    gtk_box_pack_start(
        g_type_check_instance_cast(vbox1 as *mut GTypeInstance, gtk_box_get_type())
            as *mut libc::c_void as *mut GtkBox,
        hpaned1,
        (0 as libc::c_int == 0) as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
        0 as libc::c_int as guint,
    );
    gtk_paned_set_position(
        g_type_check_instance_cast(hpaned1 as *mut GTypeInstance, gtk_paned_get_type())
            as *mut libc::c_void as *mut GtkPaned,
        0 as libc::c_int,
    );
    vbox2 = gtk_vbox_new(0 as libc::c_int, 0 as libc::c_int);
    gtk_widget_show(vbox2);
    gtk_paned_pack1(
        g_type_check_instance_cast(hpaned1 as *mut GTypeInstance, gtk_paned_get_type())
            as *mut libc::c_void as *mut GtkPaned,
        vbox2,
        (0 as libc::c_int == 0) as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    hbox2 = gtk_hbox_new(0 as libc::c_int, 0 as libc::c_int);
    gtk_widget_show(hbox2);
    gtk_box_pack_start(
        g_type_check_instance_cast(vbox2 as *mut GTypeInstance, gtk_box_get_type())
            as *mut libc::c_void as *mut GtkBox,
        hbox2,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int as guint,
    );
    drawingarea2 = gtk_drawing_area_new();
    gtk_widget_show(drawingarea2);
    gtk_box_pack_start(
        g_type_check_instance_cast(hbox2 as *mut GTypeInstance, gtk_box_get_type())
            as *mut libc::c_void as *mut GtkBox,
        drawingarea2,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int as guint,
    );
    gtk_widget_set_size_request(drawingarea2, 100 as libc::c_int, 100 as libc::c_int);
    scrolledwindow3 = gtk_scrolled_window_new(
        0 as *mut GtkAdjustment,
        0 as *mut GtkAdjustment,
    );
    gtk_widget_show(scrolledwindow3);
    gtk_box_pack_end(
        g_type_check_instance_cast(hbox2 as *mut GTypeInstance, gtk_box_get_type())
            as *mut libc::c_void as *mut GtkBox,
        scrolledwindow3,
        (0 as libc::c_int == 0) as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
        1 as libc::c_int as guint,
    );
    gtk_scrolled_window_set_policy(
        g_type_check_instance_cast(
            scrolledwindow3 as *mut GTypeInstance,
            gtk_scrolled_window_get_type(),
        ) as *mut libc::c_void as *mut GtkScrolledWindow,
        GTK_POLICY_AUTOMATIC,
        GTK_POLICY_AUTOMATIC,
    );
    gtk_scrolled_window_set_shadow_type(
        g_type_check_instance_cast(
            scrolledwindow3 as *mut GTypeInstance,
            gtk_scrolled_window_get_type(),
        ) as *mut libc::c_void as *mut GtkScrolledWindow,
        GTK_SHADOW_IN,
    );
    treeview1 = gtk_tree_view_new();
    gtk_widget_show(treeview1);
    gtk_container_add(
        g_type_check_instance_cast(
            scrolledwindow3 as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut libc::c_void as *mut GtkContainer,
        treeview1,
    );
    gtk_tree_view_set_headers_visible(
        g_type_check_instance_cast(
            treeview1 as *mut GTypeInstance,
            gtk_tree_view_get_type(),
        ) as *mut libc::c_void as *mut GtkTreeView,
        0 as libc::c_int,
    );
    toolbar1 = gtk_toolbar_new();
    gtk_widget_show(toolbar1);
    gtk_box_pack_start(
        g_type_check_instance_cast(vbox2 as *mut GTypeInstance, gtk_box_get_type())
            as *mut libc::c_void as *mut GtkBox,
        toolbar1,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int as guint,
    );
    gtk_toolbar_set_style(
        g_type_check_instance_cast(
            toolbar1 as *mut GTypeInstance,
            gtk_toolbar_get_type(),
        ) as *mut libc::c_void as *mut GtkToolbar,
        GTK_TOOLBAR_ICONS,
    );
    toolitem1 = gtk_tool_item_new() as *mut GtkWidget;
    gtk_widget_show(toolitem1);
    gtk_container_add(
        g_type_check_instance_cast(
            toolbar1 as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut libc::c_void as *mut GtkContainer,
        toolitem1,
    );
    label1 = gtk_label_new(b"type - name\0" as *const u8 as *const libc::c_char);
    gtk_widget_show(label1);
    gtk_container_add(
        g_type_check_instance_cast(
            toolitem1 as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut libc::c_void as *mut GtkContainer,
        label1,
    );
    toolitem2 = gtk_tool_item_new() as *mut GtkWidget;
    gtk_widget_show(toolitem2);
    gtk_tool_item_set_expand(
        g_type_check_instance_cast(
            toolitem2 as *mut GTypeInstance,
            gtk_tool_item_get_type(),
        ) as *mut libc::c_void as *mut GtkToolItem,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    gtk_container_add(
        g_type_check_instance_cast(
            toolbar1 as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut libc::c_void as *mut GtkContainer,
        toolitem2,
    );
    label2 = gtk_label_new(b"\0" as *const u8 as *const libc::c_char);
    gtk_widget_show(label2);
    gtk_container_add(
        g_type_check_instance_cast(
            toolitem2 as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut libc::c_void as *mut GtkContainer,
        label2,
    );
    toolitem3 = gtk_tool_item_new() as *mut GtkWidget;
    gtk_widget_show(toolitem3);
    gtk_container_add(
        g_type_check_instance_cast(
            toolbar1 as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut libc::c_void as *mut GtkContainer,
        toolitem3,
    );
    button1 = gtk_button_new_with_mnemonic(
        b"Delete\0" as *const u8 as *const libc::c_char,
    );
    gtk_widget_show(button1);
    gtk_container_add(
        g_type_check_instance_cast(
            toolitem3 as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut libc::c_void as *mut GtkContainer,
        button1,
    );
    gtk_button_set_focus_on_click(
        g_type_check_instance_cast(button1 as *mut GTypeInstance, gtk_button_get_type())
            as *mut libc::c_void as *mut GtkButton,
        0 as libc::c_int,
    );
    scrolledwindow4 = gtk_scrolled_window_new(
        0 as *mut GtkAdjustment,
        0 as *mut GtkAdjustment,
    );
    gtk_widget_show(scrolledwindow4);
    gtk_box_pack_start(
        g_type_check_instance_cast(vbox2 as *mut GTypeInstance, gtk_box_get_type())
            as *mut libc::c_void as *mut GtkBox,
        scrolledwindow4,
        (0 as libc::c_int == 0) as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
        1 as libc::c_int as guint,
    );
    gtk_scrolled_window_set_policy(
        g_type_check_instance_cast(
            scrolledwindow4 as *mut GTypeInstance,
            gtk_scrolled_window_get_type(),
        ) as *mut libc::c_void as *mut GtkScrolledWindow,
        GTK_POLICY_AUTOMATIC,
        GTK_POLICY_AUTOMATIC,
    );
    gtk_scrolled_window_set_shadow_type(
        g_type_check_instance_cast(
            scrolledwindow4 as *mut GTypeInstance,
            gtk_scrolled_window_get_type(),
        ) as *mut libc::c_void as *mut GtkScrolledWindow,
        GTK_SHADOW_IN,
    );
    treeview2 = gtk_tree_view_new();
    gtk_widget_show(treeview2);
    gtk_container_add(
        g_type_check_instance_cast(
            scrolledwindow4 as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut libc::c_void as *mut GtkContainer,
        treeview2,
    );
    gtk_tree_view_set_headers_visible(
        g_type_check_instance_cast(
            treeview2 as *mut GTypeInstance,
            gtk_tree_view_get_type(),
        ) as *mut libc::c_void as *mut GtkTreeView,
        0 as libc::c_int,
    );
    gtk_tree_view_set_rules_hint(
        g_type_check_instance_cast(
            treeview2 as *mut GTypeInstance,
            gtk_tree_view_get_type(),
        ) as *mut libc::c_void as *mut GtkTreeView,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    drawingarea1 = gtk_drawing_area_new();
    gtk_widget_show(drawingarea1);
    gtk_paned_pack2(
        g_type_check_instance_cast(hpaned1 as *mut GTypeInstance, gtk_paned_get_type())
            as *mut libc::c_void as *mut GtkPaned,
        drawingarea1,
        (0 as libc::c_int == 0) as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    gtk_widget_set_size_request(drawingarea1, 300 as libc::c_int, 300 as libc::c_int);
    gtk_widget_set_events(
        drawingarea1,
        GDK_EXPOSURE_MASK as libc::c_int | GDK_POINTER_MOTION_MASK as libc::c_int
            | GDK_BUTTON_MOTION_MASK as libc::c_int
            | GDK_BUTTON_PRESS_MASK as libc::c_int
            | GDK_BUTTON_RELEASE_MASK as libc::c_int
            | GDK_ENTER_NOTIFY_MASK as libc::c_int | GDK_LEAVE_NOTIFY_MASK as libc::c_int,
    );
    g_signal_connect_data(
        window1 as gpointer,
        b"delete_event\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut GtkWidget, *mut GdkEvent, gpointer) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                on_window1_delete_event
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEvent,
                        gpointer,
                    ) -> gboolean,
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_signal_connect_data(
        new1 as gpointer,
        b"activate\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> ()>,
            GCallback,
        >(
            Some(
                on_new1_activate
                    as unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> (),
            ),
        ),
        g_type_check_instance_cast(window1 as *mut GTypeInstance, gtk_object_get_type())
            as *mut libc::c_void as *mut GtkObject as gpointer,
        None,
        G_CONNECT_SWAPPED,
    );
    g_signal_connect_data(
        open1 as gpointer,
        b"activate\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> ()>,
            GCallback,
        >(
            Some(
                on_open1_activate
                    as unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> (),
            ),
        ),
        g_type_check_instance_cast(window1 as *mut GTypeInstance, gtk_object_get_type())
            as *mut libc::c_void as *mut GtkObject as gpointer,
        None,
        G_CONNECT_SWAPPED,
    );
    g_signal_connect_data(
        save1 as gpointer,
        b"activate\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> ()>,
            GCallback,
        >(
            Some(
                on_save1_activate
                    as unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> (),
            ),
        ),
        g_type_check_instance_cast(window1 as *mut GTypeInstance, gtk_object_get_type())
            as *mut libc::c_void as *mut GtkObject as gpointer,
        None,
        G_CONNECT_SWAPPED,
    );
    g_signal_connect_data(
        save_as1 as gpointer,
        b"activate\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> ()>,
            GCallback,
        >(
            Some(
                on_save_as1_activate
                    as unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> (),
            ),
        ),
        g_type_check_instance_cast(window1 as *mut GTypeInstance, gtk_object_get_type())
            as *mut libc::c_void as *mut GtkObject as gpointer,
        None,
        G_CONNECT_SWAPPED,
    );
    g_signal_connect_data(
        quit1 as gpointer,
        b"activate\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> ()>,
            GCallback,
        >(
            Some(
                on_quit1_activate
                    as unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> (),
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_signal_connect_data(
        cut1 as gpointer,
        b"activate\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> ()>,
            GCallback,
        >(
            Some(
                on_cut1_activate
                    as unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> (),
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_signal_connect_data(
        copy1 as gpointer,
        b"activate\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> ()>,
            GCallback,
        >(
            Some(
                on_copy1_activate
                    as unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> (),
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_signal_connect_data(
        paste1 as gpointer,
        b"activate\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> ()>,
            GCallback,
        >(
            Some(
                on_paste1_activate
                    as unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> (),
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_signal_connect_data(
        delete1 as gpointer,
        b"activate\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> ()>,
            GCallback,
        >(
            Some(
                on_delete1_activate
                    as unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> (),
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_signal_connect_data(
        about1 as gpointer,
        b"activate\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> ()>,
            GCallback,
        >(
            Some(
                on_about1_activate
                    as unsafe extern "C" fn(*mut GtkMenuItem, gpointer) -> (),
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_signal_connect_data(
        drawingarea2 as gpointer,
        b"expose_event\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventExpose,
                    gpointer,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                on_drawingarea2_expose_event
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventExpose,
                        gpointer,
                    ) -> gboolean,
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_signal_connect_data(
        drawingarea2 as gpointer,
        b"motion_notify_event\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventMotion,
                    gpointer,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                on_drawingarea2_motion_notify_event
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventMotion,
                        gpointer,
                    ) -> gboolean,
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_signal_connect_data(
        button1 as gpointer,
        b"button_press_event\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventButton,
                    gpointer,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                on_button1_button_press_event
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventButton,
                        gpointer,
                    ) -> gboolean,
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_signal_connect_data(
        drawingarea1 as gpointer,
        b"expose_event\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventExpose,
                    gpointer,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                on_drawingarea1_expose_event
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventExpose,
                        gpointer,
                    ) -> gboolean,
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_signal_connect_data(
        drawingarea1 as gpointer,
        b"motion_notify_event\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventMotion,
                    gpointer,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                on_drawingarea1_motion_notify_event
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventMotion,
                        gpointer,
                    ) -> gboolean,
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_signal_connect_data(
        drawingarea1 as gpointer,
        b"configure_event\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventConfigure,
                    gpointer,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                on_drawingarea1_configure_event
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventConfigure,
                        gpointer,
                    ) -> gboolean,
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_signal_connect_data(
        drawingarea1 as gpointer,
        b"button_press_event\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventButton,
                    gpointer,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                on_drawingarea1_button_press_event
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventButton,
                        gpointer,
                    ) -> gboolean,
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_signal_connect_data(
        drawingarea1 as gpointer,
        b"button_release_event\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventButton,
                    gpointer,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                on_drawingarea1_button_release_event
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventButton,
                        gpointer,
                    ) -> gboolean,
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_signal_connect_data(
        drawingarea1 as gpointer,
        b"scroll_event\0" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut GtkWidget, *mut GdkEvent, gpointer) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                on_drawingarea1_scroll_event
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEvent,
                        gpointer,
                    ) -> gboolean,
            ),
        ),
        0 as *mut libc::c_void,
        None,
        0 as GConnectFlags,
    );
    g_object_set_data(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"window1\0" as *const u8 as *const libc::c_char,
        window1 as gpointer,
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"vbox1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(vbox1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"menubar1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(menubar1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"menuitem4\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(menuitem4) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"menu4\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(menu4) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"new1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(new1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"open1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(open1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"save1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(save1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"save_as1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(save_as1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"separatormenuitem1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(separatormenuitem1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"quit1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(quit1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"menuitem5\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(menuitem5) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"menu5\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(menu5) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"cut1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(cut1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"copy1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(copy1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"paste1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(paste1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"delete1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(delete1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"menuitem6\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(menuitem6) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"menu6\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(menu6) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"menuitem7\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(menuitem7) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"menu7\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(menu7) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"about1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(about1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"hpaned1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(hpaned1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"vbox2\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(vbox2) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"hbox2\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(hbox2) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"drawingarea2\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(drawingarea2) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"scrolledwindow3\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(scrolledwindow3) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"treeview1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(treeview1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"toolbar1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(toolbar1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"toolitem1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(toolitem1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"label1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(label1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"toolitem2\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(toolitem2) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"label2\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(label2) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"toolitem3\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(toolitem3) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"button1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(button1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"scrolledwindow4\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(scrolledwindow4) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"treeview2\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(treeview2) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    g_object_set_data_full(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"drawingarea1\0" as *const u8 as *const libc::c_char,
        gtk_widget_ref(drawingarea1) as gpointer,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget) -> ()>,
            GDestroyNotify,
        >(Some(gtk_widget_unref as unsafe extern "C" fn(*mut GtkWidget) -> ())),
    );
    gtk_window_add_accel_group(
        g_type_check_instance_cast(window1 as *mut GTypeInstance, gtk_window_get_type())
            as *mut libc::c_void as *mut GtkWindow,
        accel_group,
    );
    return window1;
}
