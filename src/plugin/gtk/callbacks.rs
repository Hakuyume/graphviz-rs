#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _GData;
    pub type _cairo;
    pub type _cairo_font_options;
    pub type _PangoFontDescription;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _GdkAtom;
    pub type _GdkRegion;
    pub type _GtkWindowGeometryInfo;
    pub type _GtkTreeModel;
    pub type _GtkFileChooser;
    pub type gvloadimage_engine_s;
    pub type gvrender_engine_s;
    pub type GVC_s;
    fn g_free(mem: gpointer);
    fn g_type_check_instance_cast(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> *mut GTypeInstance;
    fn g_object_get_data(object: *mut GObject, key: *const gchar) -> gpointer;
    fn g_object_set_data_full(
        object: *mut GObject,
        key: *const gchar,
        data: gpointer,
        destroy: GDestroyNotify,
    );
    fn cairo_destroy(cr: *mut cairo_t);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn gdk_cairo_create(drawable: *mut GdkDrawable) -> *mut cairo_t;
    fn gtk_widget_get_type() -> GType;
    fn gtk_widget_destroy(widget: *mut GtkWidget);
    fn gtk_widget_queue_draw(widget: *mut GtkWidget);
    fn gtk_widget_get_toplevel(widget: *mut GtkWidget) -> *mut GtkWidget;
    fn gtk_window_get_type() -> GType;
    fn gtk_dialog_get_type() -> GType;
    fn gtk_dialog_run(dialog: *mut GtkDialog) -> gint;
    fn gtk_show_about_dialog(
        parent: *mut GtkWindow,
        first_property_name: *const gchar,
        _: ...
    );
    fn gtk_list_store_get_type() -> GType;
    fn gtk_list_store_clear(list_store: *mut GtkListStore);
    fn gtk_file_chooser_get_type() -> GType;
    fn gtk_file_chooser_get_filename(chooser: *mut GtkFileChooser) -> *mut gchar;
    fn gtk_file_chooser_set_filename(
        chooser: *mut GtkFileChooser,
        filename: *const libc::c_char,
    ) -> gboolean;
    fn gtk_file_chooser_dialog_new(
        title: *const gchar,
        parent: *mut GtkWindow,
        action: GtkFileChooserAction,
        first_button_text: *const gchar,
        _: ...
    ) -> *mut GtkWidget;
    fn gtk_main_quit();
}
pub type size_t = libc::c_ulong;
pub type gint8 = libc::c_schar;
pub type guint8 = libc::c_uchar;
pub type gint16 = libc::c_short;
pub type guint16 = libc::c_ushort;
pub type guint32 = libc::c_uint;
pub type gsize = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type gchar = libc::c_char;
pub type gshort = libc::c_short;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type gushort = libc::c_ushort;
pub type guint = libc::c_uint;
pub type gdouble = libc::c_double;
pub type gpointer = *mut libc::c_void;
pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
pub type GInitiallyUnowned = _GObject;
pub type cairo_t = _cairo;
pub type cairo_font_options_t = _cairo_font_options;
pub type uint64_t = __uint64_t;
pub type PangoFontDescription = _PangoFontDescription;
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
pub type GdkDrawable = _GdkDrawable;
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
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
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub type GtkSortType = libc::c_uint;
pub const GTK_SORT_DESCENDING: GtkSortType = 1;
pub const GTK_SORT_ASCENDING: GtkSortType = 0;
pub type GtkObject = _GtkObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkObject {
    pub parent_instance: GInitiallyUnowned,
    pub flags: guint32,
}
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
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
pub type C2RustUnnamed_0 = libc::c_int;
pub const GTK_RESPONSE_HELP: C2RustUnnamed_0 = -11;
pub const GTK_RESPONSE_APPLY: C2RustUnnamed_0 = -10;
pub const GTK_RESPONSE_NO: C2RustUnnamed_0 = -9;
pub const GTK_RESPONSE_YES: C2RustUnnamed_0 = -8;
pub const GTK_RESPONSE_CLOSE: C2RustUnnamed_0 = -7;
pub const GTK_RESPONSE_CANCEL: C2RustUnnamed_0 = -6;
pub const GTK_RESPONSE_OK: C2RustUnnamed_0 = -5;
pub const GTK_RESPONSE_DELETE_EVENT: C2RustUnnamed_0 = -4;
pub const GTK_RESPONSE_ACCEPT: C2RustUnnamed_0 = -3;
pub const GTK_RESPONSE_REJECT: C2RustUnnamed_0 = -2;
pub const GTK_RESPONSE_NONE: C2RustUnnamed_0 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkDialog {
    pub window: GtkWindow,
    pub vbox: *mut GtkWidget,
    pub action_area: *mut GtkWidget,
    pub separator: *mut GtkWidget,
}
pub type GtkDialog = _GtkDialog;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkTreeIter {
    pub stamp: gint,
    pub user_data: gpointer,
    pub user_data2: gpointer,
    pub user_data3: gpointer,
}
pub type GtkTreeIter = _GtkTreeIter;
pub type GtkTreeModel = _GtkTreeModel;
pub type GtkTreeIterCompareFunc = Option::<
    unsafe extern "C" fn(
        *mut GtkTreeModel,
        *mut GtkTreeIter,
        *mut GtkTreeIter,
        gpointer,
    ) -> gint,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkItem {
    pub bin: GtkBin,
}
pub type GtkItem = _GtkItem;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _GtkListStore {
    pub parent: GObject,
    pub stamp: gint,
    pub seq: gpointer,
    pub _gtk_reserved1: gpointer,
    pub sort_list: *mut GList,
    pub n_columns: gint,
    pub sort_column_id: gint,
    pub order: GtkSortType,
    pub column_headers: *mut GType,
    pub length: gint,
    pub default_sort_func: GtkTreeIterCompareFunc,
    pub default_sort_data: gpointer,
    pub default_sort_destroy: GDestroyNotify,
    #[bitfield(name = "columns_dirty", ty = "guint", bits = "0..=0")]
    pub columns_dirty: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type GtkListStore = _GtkListStore;
pub type GtkFileChooser = _GtkFileChooser;
pub type GtkFileChooserAction = libc::c_uint;
pub const GTK_FILE_CHOOSER_ACTION_CREATE_FOLDER: GtkFileChooserAction = 3;
pub const GTK_FILE_CHOOSER_ACTION_SELECT_FOLDER: GtkFileChooserAction = 2;
pub const GTK_FILE_CHOOSER_ACTION_SAVE: GtkFileChooserAction = 1;
pub const GTK_FILE_CHOOSER_ACTION_OPEN: GtkFileChooserAction = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pointf_s {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type pointf = pointf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct box_0 {
    pub LL: point,
    pub UR: point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct boxf {
    pub LL: pointf,
    pub UR: pointf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GVJ_s {
    pub gvc: *mut GVC_t,
    pub next: *mut GVJ_t,
    pub next_active: *mut GVJ_t,
    pub common: *mut GVCOMMON_t,
    pub obj: *mut obj_state_t,
    pub input_filename: *mut libc::c_char,
    pub graph_index: libc::c_int,
    pub layout_type: *const libc::c_char,
    pub output_filename: *const libc::c_char,
    pub output_file: *mut FILE,
    pub output_data: *mut libc::c_char,
    pub output_data_allocated: libc::c_uint,
    pub output_data_position: libc::c_uint,
    pub output_langname: *const libc::c_char,
    pub output_lang: libc::c_int,
    pub render: gvplugin_active_render_t,
    pub device: gvplugin_active_device_t,
    pub loadimage: gvplugin_active_loadimage_t,
    pub callbacks: *mut gvdevice_callbacks_t,
    pub device_dpi: pointf,
    pub device_sets_dpi: bool,
    pub display: *mut libc::c_void,
    pub screen: libc::c_int,
    pub context: *mut libc::c_void,
    pub external_context: bool,
    pub imagedata: *mut libc::c_char,
    pub flags: libc::c_int,
    pub numLayers: libc::c_int,
    pub layerNum: libc::c_int,
    pub pagesArraySize: point,
    pub pagesArrayFirst: point,
    pub pagesArrayMajor: point,
    pub pagesArrayMinor: point,
    pub pagesArrayElem: point,
    pub numPages: libc::c_int,
    pub bb: boxf,
    pub pad: pointf,
    pub clip: boxf,
    pub pageBox: boxf,
    pub pageSize: pointf,
    pub focus: pointf,
    pub zoom: libc::c_double,
    pub rotation: libc::c_int,
    pub view: pointf,
    pub canvasBox: boxf,
    pub margin: pointf,
    pub dpi: pointf,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub pageBoundingBox: box_0,
    pub boundingBox: box_0,
    pub scale: pointf,
    pub translation: pointf,
    pub devscale: pointf,
    pub fit_mode: bool,
    pub needs_refresh: bool,
    pub click: bool,
    pub has_grown: bool,
    pub has_been_rendered: bool,
    pub button: libc::c_uchar,
    pub pointer: pointf,
    pub oldpointer: pointf,
    pub current_obj: *mut libc::c_void,
    pub selected_obj: *mut libc::c_void,
    pub active_tooltip: *mut libc::c_char,
    pub selected_href: *mut libc::c_char,
    pub selected_obj_type_name: gv_argvlist_t,
    pub selected_obj_attributes: gv_argvlist_t,
    pub window: *mut libc::c_void,
    pub keybindings: *mut gvevent_key_binding_t,
    pub numkeys: libc::c_int,
    pub keycodes: *mut libc::c_void,
}
pub type gvevent_key_binding_t = gvevent_key_binding_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvevent_key_binding_s {
    pub keystring: *mut libc::c_char,
    pub callback: gvevent_key_callback_t,
}
pub type gvevent_key_callback_t = Option::<
    unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int,
>;
pub type GVJ_t = GVJ_s;
pub type gv_argvlist_t = gv_argvlist_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gv_argvlist_s {
    pub argv: *mut *mut libc::c_char,
    pub argc: libc::c_int,
    pub alloc: libc::c_int,
}
pub type gvdevice_callbacks_t = gvdevice_callbacks_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvdevice_callbacks_s {
    pub refresh: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub button_press: Option::<
        unsafe extern "C" fn(*mut GVJ_t, libc::c_int, pointf) -> (),
    >,
    pub button_release: Option::<
        unsafe extern "C" fn(*mut GVJ_t, libc::c_int, pointf) -> (),
    >,
    pub motion: Option::<unsafe extern "C" fn(*mut GVJ_t, pointf) -> ()>,
    pub modify: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char, *const libc::c_char) -> (),
    >,
    pub del: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub read: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char, *const libc::c_char) -> (),
    >,
    pub layout: Option::<unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char) -> ()>,
    pub render: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char, *const libc::c_char) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_active_loadimage_t {
    pub engine: *mut gvloadimage_engine_t,
    pub id: libc::c_int,
    pub type_0: *const libc::c_char,
}
pub type gvloadimage_engine_t = gvloadimage_engine_s;
pub type gvplugin_active_device_t = gvplugin_active_device_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_active_device_s {
    pub engine: *mut gvdevice_engine_t,
    pub id: libc::c_int,
    pub features: *mut gvdevice_features_t,
    pub type_0: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvdevice_features_t {
    pub flags: libc::c_int,
    pub default_margin: pointf,
    pub default_pagesize: pointf,
    pub default_dpi: pointf,
}
pub type gvdevice_engine_t = gvdevice_engine_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvdevice_engine_s {
    pub initialize: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub format: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub finalize: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
}
pub type gvplugin_active_render_t = gvplugin_active_render_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_active_render_s {
    pub engine: *mut gvrender_engine_t,
    pub id: libc::c_int,
    pub features: *mut gvrender_features_t,
    pub type_0: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvrender_features_t {
    pub flags: libc::c_int,
    pub default_pad: libc::c_double,
    pub knowncolors: *mut *mut libc::c_char,
    pub sz_knowncolors: libc::c_int,
    pub color_type: color_type_t,
}
pub type color_type_t = libc::c_uint;
pub const COLOR_INDEX: color_type_t = 6;
pub const COLOR_STRING: color_type_t = 5;
pub const RGBA_DOUBLE: color_type_t = 4;
pub const CMYK_BYTE: color_type_t = 3;
pub const RGBA_WORD: color_type_t = 2;
pub const RGBA_BYTE: color_type_t = 1;
pub const HSVA_DOUBLE: color_type_t = 0;
pub type gvrender_engine_t = gvrender_engine_s;
pub type obj_state_t = obj_state_s;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct obj_state_s {
    pub parent: *mut obj_state_t,
    pub type_0: obj_type,
    pub u: C2RustUnnamed_2,
    pub emit_state: emit_state_t,
    pub pencolor: gvcolor_t,
    pub fillcolor: gvcolor_t,
    pub stopcolor: gvcolor_t,
    pub gradient_angle: libc::c_int,
    pub gradient_frac: libc::c_float,
    pub pen: pen_type,
    pub fill: fill_type,
    pub penwidth: libc::c_double,
    pub rawstyle: *mut *mut libc::c_char,
    pub z: libc::c_double,
    pub tail_z: libc::c_double,
    pub head_z: libc::c_double,
    pub label: *mut libc::c_char,
    pub xlabel: *mut libc::c_char,
    pub taillabel: *mut libc::c_char,
    pub headlabel: *mut libc::c_char,
    pub url: *mut libc::c_char,
    pub id: *mut libc::c_char,
    pub labelurl: *mut libc::c_char,
    pub tailurl: *mut libc::c_char,
    pub headurl: *mut libc::c_char,
    pub tooltip: *mut libc::c_char,
    pub labeltooltip: *mut libc::c_char,
    pub tailtooltip: *mut libc::c_char,
    pub headtooltip: *mut libc::c_char,
    pub target: *mut libc::c_char,
    pub labeltarget: *mut libc::c_char,
    pub tailtarget: *mut libc::c_char,
    pub headtarget: *mut libc::c_char,
    #[bitfield(name = "explicit_tooltip", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "explicit_tailtooltip", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "explicit_headtooltip", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "explicit_labeltooltip", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "explicit_tailtarget", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "explicit_headtarget", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "explicit_edgetarget", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "explicit_tailurl", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "explicit_headurl", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "labeledgealigned", ty = "libc::c_uint", bits = "9..=9")]
    pub explicit_tooltip_explicit_tailtooltip_explicit_headtooltip_explicit_labeltooltip_explicit_tailtarget_explicit_headtarget_explicit_edgetarget_explicit_tailurl_explicit_headurl_labeledgealigned: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub url_map_shape: map_shape_t,
    pub url_map_n: libc::c_int,
    pub url_map_p: *mut pointf,
    pub url_bsplinemap_poly_n: libc::c_int,
    pub url_bsplinemap_n: *mut libc::c_int,
    pub url_bsplinemap_p: *mut pointf,
    pub tailendurl_map_n: libc::c_int,
    pub tailendurl_map_p: *mut pointf,
    pub headendurl_map_n: libc::c_int,
    pub headendurl_map_p: *mut pointf,
}
pub type map_shape_t = libc::c_uint;
pub const MAP_POLYGON: map_shape_t = 2;
pub const MAP_CIRCLE: map_shape_t = 1;
pub const MAP_RECTANGLE: map_shape_t = 0;
pub type fill_type = libc::c_uint;
pub const FILL_RADIAL: fill_type = 3;
pub const FILL_LINEAR: fill_type = 2;
pub const FILL_SOLID: fill_type = 1;
pub const FILL_NONE: fill_type = 0;
pub type pen_type = libc::c_uint;
pub const PEN_SOLID: pen_type = 3;
pub const PEN_DOTTED: pen_type = 2;
pub const PEN_DASHED: pen_type = 1;
pub const PEN_NONE: pen_type = 0;
pub type gvcolor_t = color_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_s {
    pub u: C2RustUnnamed_1,
    pub type_0: color_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub RGBA: [libc::c_double; 4],
    pub HSVA: [libc::c_double; 4],
    pub rgba: [libc::c_uchar; 4],
    pub cmyk: [libc::c_uchar; 4],
    pub rrggbbaa: [libc::c_int; 4],
    pub string: *mut libc::c_char,
    pub index: libc::c_int,
}
pub type emit_state_t = libc::c_uint;
pub const EMIT_ELABEL: emit_state_t = 11;
pub const EMIT_NLABEL: emit_state_t = 10;
pub const EMIT_EDRAW: emit_state_t = 9;
pub const EMIT_NDRAW: emit_state_t = 8;
pub const EMIT_HLABEL: emit_state_t = 7;
pub const EMIT_TLABEL: emit_state_t = 6;
pub const EMIT_CLABEL: emit_state_t = 5;
pub const EMIT_GLABEL: emit_state_t = 4;
pub const EMIT_HDRAW: emit_state_t = 3;
pub const EMIT_TDRAW: emit_state_t = 2;
pub const EMIT_CDRAW: emit_state_t = 1;
pub const EMIT_GDRAW: emit_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub g: *mut graph_t,
    pub sg: *mut graph_t,
    pub n: *mut node_t,
    pub e: *mut edge_t,
}
pub type edge_t = Agedge_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agedge_s {
    pub base: Agobj_t,
    pub id_link: Dtlink_t,
    pub seq_link: Dtlink_t,
    pub node: *mut Agnode_t,
}
pub type Agnode_t = Agnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agnode_s {
    pub base: Agobj_t,
    pub root: *mut Agraph_t,
    pub mainsub: Agsubnode_t,
}
pub type Agsubnode_t = Agsubnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agsubnode_s {
    pub seq_link: Dtlink_t,
    pub id_link: Dtlink_t,
    pub node: *mut Agnode_t,
    pub in_id: *mut Dtlink_t,
    pub out_id: *mut Dtlink_t,
    pub in_seq: *mut Dtlink_t,
    pub out_seq: *mut Dtlink_t,
}
pub type Dtlink_t = _dtlink_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtlink_s {
    pub right: *mut Dtlink_t,
    pub hl: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _hash: libc::c_uint,
    pub _left: *mut Dtlink_t,
}
pub type Agraph_t = Agraph_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agraph_s {
    pub base: Agobj_t,
    pub desc: Agdesc_t,
    pub link: Dtlink_t,
    pub n_seq: *mut Dict_t,
    pub n_id: *mut Dict_t,
    pub e_seq: *mut Dict_t,
    pub e_id: *mut Dict_t,
    pub g_dict: *mut Dict_t,
    pub parent: *mut Agraph_t,
    pub root: *mut Agraph_t,
    pub clos: *mut Agclos_t,
}
pub type Agclos_t = Agclos_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agclos_s {
    pub disc: Agdisc_t,
    pub state: Agdstate_t,
    pub strdict: *mut Dict_t,
    pub seq: [uint64_t; 3],
    pub cb: *mut Agcbstack_t,
    pub callbacks_enabled: libc::c_uchar,
    pub lookup_by_name: [*mut Dict_t; 3],
    pub lookup_by_id: [*mut Dict_t; 3],
}
pub type Dict_t = _dt_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dt_s {
    pub searchf: Dtsearch_f,
    pub disc: *mut Dtdisc_t,
    pub data: *mut Dtdata_t,
    pub memoryf: Dtmemory_f,
    pub meth: *mut Dtmethod_t,
    pub type_0: libc::c_int,
    pub nview: libc::c_int,
    pub view: *mut Dt_t,
    pub walk: *mut Dt_t,
    pub user: *mut libc::c_void,
}
pub type Dt_t = _dt_s;
pub type Dtmethod_t = _dtmethod_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtmethod_s {
    pub searchf: Dtsearch_f,
    pub type_0: libc::c_int,
}
pub type Dtsearch_f = Option::<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, libc::c_int) -> *mut libc::c_void,
>;
pub type Dtmemory_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        *mut libc::c_void,
        size_t,
        *mut Dtdisc_t,
    ) -> *mut libc::c_void,
>;
pub type Dtdisc_t = _dtdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtdisc_s {
    pub key: libc::c_int,
    pub size: libc::c_int,
    pub link: libc::c_int,
    pub makef: Dtmake_f,
    pub freef: Dtfree_f,
    pub comparf: Dtcompar_f,
    pub hashf: Dthash_f,
    pub memoryf: Dtmemory_f,
    pub eventf: Dtevent_f,
}
pub type Dtevent_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        libc::c_int,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> libc::c_int,
>;
pub type Dthash_f = Option::<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> libc::c_uint,
>;
pub type Dtcompar_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        *mut libc::c_void,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> libc::c_int,
>;
pub type Dtfree_f = Option::<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> (),
>;
pub type Dtmake_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> *mut libc::c_void,
>;
pub type Dtdata_t = _dtdata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtdata_s {
    pub type_0: libc::c_int,
    pub here: *mut Dtlink_t,
    pub hh: C2RustUnnamed_4,
    pub ntab: libc::c_int,
    pub size: libc::c_int,
    pub loop_0: libc::c_int,
    pub minp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _htab: *mut *mut Dtlink_t,
    pub _head: *mut Dtlink_t,
}
pub type Agcbstack_t = Agcbstack_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agcbstack_s {
    pub f: *mut Agcbdisc_t,
    pub state: *mut libc::c_void,
    pub prev: *mut Agcbstack_t,
}
pub type Agcbdisc_t = Agcbdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agcbdisc_s {
    pub graph: C2RustUnnamed_5,
    pub node: C2RustUnnamed_5,
    pub edge: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub ins: agobjfn_t,
    pub mod_0: agobjupdfn_t,
    pub del: agobjfn_t,
}
pub type agobjfn_t = Option::<
    unsafe extern "C" fn(*mut Agraph_t, *mut Agobj_t, *mut libc::c_void) -> (),
>;
pub type Agobj_t = Agobj_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agobj_s {
    pub tag: Agtag_t,
    pub data: *mut Agrec_t,
}
pub type Agrec_t = Agrec_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agrec_s {
    pub name: *mut libc::c_char,
    pub next: *mut Agrec_t,
}
pub type Agtag_t = Agtag_s;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct Agtag_s {
    #[bitfield(name = "objtype", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "mtflock", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "attrwf", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "seq", ty = "libc::c_uint", bits = "4..=31")]
    pub objtype_mtflock_attrwf_seq: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub id: IDTYPE,
}
pub type IDTYPE = uint64_t;
pub type agobjupdfn_t = Option::<
    unsafe extern "C" fn(
        *mut Agraph_t,
        *mut Agobj_t,
        *mut libc::c_void,
        *mut Agsym_t,
    ) -> (),
>;
pub type Agsym_t = Agsym_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agsym_s {
    pub link: Dtlink_t,
    pub name: *mut libc::c_char,
    pub defval: *mut libc::c_char,
    pub id: libc::c_int,
    pub kind: libc::c_uchar,
    pub fixed: libc::c_uchar,
    pub print: libc::c_uchar,
}
pub type Agdstate_t = Agdstate_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agdstate_s {
    pub mem: *mut libc::c_void,
    pub id: *mut libc::c_void,
}
pub type Agdisc_t = Agdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agdisc_s {
    pub mem: *mut Agmemdisc_t,
    pub id: *mut Agiddisc_t,
    pub io: *mut Agiodisc_t,
}
pub type Agiodisc_t = Agiodisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agiodisc_s {
    pub afread: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub putstr: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
    >,
    pub flush: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
}
pub type Agiddisc_t = Agiddisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agiddisc_s {
    pub open: Option::<
        unsafe extern "C" fn(*mut Agraph_t, *mut Agdisc_t) -> *mut libc::c_void,
    >,
    pub map: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_char,
            *mut IDTYPE,
            libc::c_int,
        ) -> libc::c_long,
    >,
    pub alloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> libc::c_long,
    >,
    pub free: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> (),
    >,
    pub print: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> *mut libc::c_char,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub idregister: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, *mut libc::c_void) -> (),
    >,
}
pub type Agmemdisc_t = Agmemdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agmemdisc_s {
    pub open: Option::<unsafe extern "C" fn(*mut Agdisc_t) -> *mut libc::c_void>,
    pub alloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub resize: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            size_t,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    pub close: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type Agdesc_t = Agdesc_s;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct Agdesc_s {
    #[bitfield(name = "directed", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "strict", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "no_loop", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "maingraph", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "flatlock", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "no_write", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "has_attrs", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "has_cmpnd", ty = "libc::c_uint", bits = "7..=7")]
    pub directed_strict_no_loop_maingraph_flatlock_no_write_has_attrs_has_cmpnd: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type node_t = Agnode_s;
pub type graph_t = Agraph_s;
pub type obj_type = libc::c_uint;
pub const EDGE_OBJTYPE: obj_type = 3;
pub const NODE_OBJTYPE: obj_type = 2;
pub const CLUSTER_OBJTYPE: obj_type = 1;
pub const ROOTGRAPH_OBJTYPE: obj_type = 0;
pub type GVCOMMON_t = GVCOMMON_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GVCOMMON_s {
    pub info: *mut *mut libc::c_char,
    pub cmdname: *mut libc::c_char,
    pub verbose: libc::c_int,
    pub config: bool,
    pub auto_outfile_names: bool,
    pub errorfn: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub show_boxes: *mut *const libc::c_char,
    pub lib: *mut *const libc::c_char,
    pub viewNum: libc::c_int,
    pub builtins: *const lt_symlist_t,
    pub demand_loading: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lt_symlist_t {
    pub name: *const libc::c_char,
    pub address: *mut libc::c_void,
}
pub type GVC_t = GVC_s;
#[no_mangle]
pub unsafe extern "C" fn on_new1_activate(
    mut menuitem: *mut GtkMenuItem,
    mut user_data: gpointer,
) {
    let mut window1: *mut GtkWindow = 0 as *mut GtkWindow;
    let mut job: *mut GVJ_t = 0 as *mut GVJ_t;
    window1 = g_type_check_instance_cast(
        menuitem as *mut GTypeInstance,
        gtk_window_get_type(),
    ) as *mut libc::c_void as *mut GtkWindow;
    job = g_object_get_data(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"job\0" as *const u8 as *const libc::c_char,
    ) as *mut GVJ_t;
    ((*(*job).callbacks).read)
        .expect(
            "non-null function pointer",
        )(job, 0 as *const libc::c_char, b"dot\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn ui_open_graph(
    mut window1: *mut GtkWindow,
    mut filename: *mut gchar,
) {
    let mut job: *mut GVJ_t = 0 as *mut GVJ_t;
    let mut dialog: *mut GtkWidget = 0 as *mut GtkWidget;
    job = g_object_get_data(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"job\0" as *const u8 as *const libc::c_char,
    ) as *mut GVJ_t;
    dialog = gtk_file_chooser_dialog_new(
        b"Open graph\0" as *const u8 as *const libc::c_char,
        window1,
        GTK_FILE_CHOOSER_ACTION_OPEN,
        b"Cancel\0" as *const u8 as *const libc::c_char,
        GTK_RESPONSE_CANCEL as libc::c_int,
        b"Open\0" as *const u8 as *const libc::c_char,
        GTK_RESPONSE_ACCEPT as libc::c_int,
        0 as *mut libc::c_void,
    );
    if !filename.is_null() {
        gtk_file_chooser_set_filename(
            g_type_check_instance_cast(
                dialog as *mut GTypeInstance,
                gtk_file_chooser_get_type(),
            ) as *mut libc::c_void as *mut GtkFileChooser,
            filename,
        );
    }
    if gtk_dialog_run(
        g_type_check_instance_cast(dialog as *mut GTypeInstance, gtk_dialog_get_type())
            as *mut libc::c_void as *mut GtkDialog,
    ) == GTK_RESPONSE_ACCEPT as libc::c_int
    {
        filename = gtk_file_chooser_get_filename(
            g_type_check_instance_cast(
                dialog as *mut GTypeInstance,
                gtk_file_chooser_get_type(),
            ) as *mut libc::c_void as *mut GtkFileChooser,
        );
    }
    gtk_widget_destroy(dialog);
    if !filename.is_null() {
        ((*(*job).callbacks).read)
            .expect(
                "non-null function pointer",
            )(job, filename, b"dot\0" as *const u8 as *const libc::c_char);
        g_object_set_data_full(
            g_type_check_instance_cast(
                window1 as *mut GTypeInstance,
                ((20 as libc::c_int) << 2 as libc::c_int) as GType,
            ) as *mut libc::c_void as *mut GObject,
            b"activefilename\0" as *const u8 as *const libc::c_char,
            filename as gpointer,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                GDestroyNotify,
            >(Some(g_free as unsafe extern "C" fn(gpointer) -> ())),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn on_open1_activate(
    mut menuitem: *mut GtkMenuItem,
    mut user_data: gpointer,
) {
    let mut window1: *mut GtkWindow = 0 as *mut GtkWindow;
    let mut filename: *mut gchar = 0 as *mut gchar;
    window1 = g_type_check_instance_cast(
        menuitem as *mut GTypeInstance,
        gtk_window_get_type(),
    ) as *mut libc::c_void as *mut GtkWindow;
    filename = g_object_get_data(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"activefilename\0" as *const u8 as *const libc::c_char,
    ) as *mut gchar;
    ui_open_graph(window1, filename);
}
unsafe extern "C" fn ui_save_graph(
    mut window1: *mut GtkWindow,
    mut filename: *mut gchar,
) {
    let mut job: *mut GVJ_t = 0 as *mut GVJ_t;
    let mut dialog: *mut GtkWidget = 0 as *mut GtkWidget;
    job = g_object_get_data(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"job\0" as *const u8 as *const libc::c_char,
    ) as *mut GVJ_t;
    dialog = gtk_file_chooser_dialog_new(
        b"Save graph as\0" as *const u8 as *const libc::c_char,
        window1,
        GTK_FILE_CHOOSER_ACTION_SAVE,
        b"Cancel\0" as *const u8 as *const libc::c_char,
        GTK_RESPONSE_CANCEL as libc::c_int,
        b"Save\0" as *const u8 as *const libc::c_char,
        GTK_RESPONSE_ACCEPT as libc::c_int,
        0 as *mut libc::c_void,
    );
    filename = g_object_get_data(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"activefilename\0" as *const u8 as *const libc::c_char,
    ) as *mut gchar;
    if !filename.is_null() {
        gtk_file_chooser_set_filename(
            g_type_check_instance_cast(
                dialog as *mut GTypeInstance,
                gtk_file_chooser_get_type(),
            ) as *mut libc::c_void as *mut GtkFileChooser,
            filename,
        );
    }
    if gtk_dialog_run(
        g_type_check_instance_cast(dialog as *mut GTypeInstance, gtk_dialog_get_type())
            as *mut libc::c_void as *mut GtkDialog,
    ) == GTK_RESPONSE_ACCEPT as libc::c_int
    {
        filename = gtk_file_chooser_get_filename(
            g_type_check_instance_cast(
                dialog as *mut GTypeInstance,
                gtk_file_chooser_get_type(),
            ) as *mut libc::c_void as *mut GtkFileChooser,
        );
    }
    gtk_widget_destroy(dialog);
    if !filename.is_null() {
        ((*(*job).callbacks).render)
            .expect(
                "non-null function pointer",
            )(job, b"dot\0" as *const u8 as *const libc::c_char, filename);
        g_object_set_data_full(
            g_type_check_instance_cast(
                window1 as *mut GTypeInstance,
                ((20 as libc::c_int) << 2 as libc::c_int) as GType,
            ) as *mut libc::c_void as *mut GObject,
            b"activefilename\0" as *const u8 as *const libc::c_char,
            filename as gpointer,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                GDestroyNotify,
            >(Some(g_free as unsafe extern "C" fn(gpointer) -> ())),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn on_save1_activate(
    mut menuitem: *mut GtkMenuItem,
    mut user_data: gpointer,
) {
    let mut window1: *mut GtkWindow = 0 as *mut GtkWindow;
    let mut filename: *mut gchar = 0 as *mut gchar;
    window1 = g_type_check_instance_cast(
        menuitem as *mut GTypeInstance,
        gtk_window_get_type(),
    ) as *mut libc::c_void as *mut GtkWindow;
    filename = g_object_get_data(
        g_type_check_instance_cast(
            window1 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"activefilename\0" as *const u8 as *const libc::c_char,
    ) as *mut gchar;
    ui_save_graph(window1, filename);
}
#[no_mangle]
pub unsafe extern "C" fn on_save_as1_activate(
    mut menuitem: *mut GtkMenuItem,
    mut user_data: gpointer,
) {
    let mut window1: *mut GtkWindow = 0 as *mut GtkWindow;
    window1 = g_type_check_instance_cast(
        menuitem as *mut GTypeInstance,
        gtk_window_get_type(),
    ) as *mut libc::c_void as *mut GtkWindow;
    ui_save_graph(window1, 0 as *mut gchar);
}
#[no_mangle]
pub unsafe extern "C" fn on_quit1_activate(
    mut menuitem: *mut GtkMenuItem,
    mut user_data: gpointer,
) {
    gtk_widget_destroy(
        g_type_check_instance_cast(
            gtk_widget_get_toplevel(
                g_type_check_instance_cast(
                    menuitem as *mut GTypeInstance,
                    gtk_widget_get_type(),
                ) as *mut libc::c_void as *mut GtkWidget,
            ) as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut libc::c_void as *mut GtkWidget,
    );
    gtk_main_quit();
}
#[no_mangle]
pub unsafe extern "C" fn on_cut1_activate(
    mut menuitem: *mut GtkMenuItem,
    mut user_data: gpointer,
) {}
#[no_mangle]
pub unsafe extern "C" fn on_copy1_activate(
    mut menuitem: *mut GtkMenuItem,
    mut user_data: gpointer,
) {}
#[no_mangle]
pub unsafe extern "C" fn on_paste1_activate(
    mut menuitem: *mut GtkMenuItem,
    mut user_data: gpointer,
) {}
#[no_mangle]
pub unsafe extern "C" fn on_delete1_activate(
    mut menuitem: *mut GtkMenuItem,
    mut user_data: gpointer,
) {}
#[no_mangle]
pub unsafe extern "C" fn on_about1_activate(
    mut menuitem: *mut GtkMenuItem,
    mut user_data: gpointer,
) {
    static mut authors: [*mut gchar; 5] = [
        b"John Ellson\0" as *const u8 as *const libc::c_char as *mut gchar,
        b"Emden Gansner\0" as *const u8 as *const libc::c_char as *mut gchar,
        b"Stephen North\0" as *const u8 as *const libc::c_char as *mut gchar,
        b"special thanks to Michael Lawrence\0" as *const u8 as *const libc::c_char
            as *mut gchar,
        0 as *const gchar as *mut gchar,
    ];
    let mut window: *mut GtkWindow = g_type_check_instance_cast(
        gtk_widget_get_toplevel(
            g_type_check_instance_cast(
                menuitem as *mut GTypeInstance,
                gtk_widget_get_type(),
            ) as *mut libc::c_void as *mut GtkWidget,
        ) as *mut GTypeInstance,
        gtk_window_get_type(),
    ) as *mut libc::c_void as *mut GtkWindow;
    gtk_show_about_dialog(
        window,
        b"name\0" as *const u8 as *const libc::c_char,
        b"DotEdit\0" as *const u8 as *const libc::c_char,
        b"program-name\0" as *const u8 as *const libc::c_char,
        b"DotEdit\0" as *const u8 as *const libc::c_char,
        b"version\0" as *const u8 as *const libc::c_char,
        b"0.1\0" as *const u8 as *const libc::c_char,
        b"copyright\0" as *const u8 as *const libc::c_char,
        b"(C) 2011 AT&T Intellectual Procerty.\0" as *const u8 as *const libc::c_char,
        b"license\0" as *const u8 as *const libc::c_char,
        b"Eclipse Public License v1.0.\0" as *const u8 as *const libc::c_char,
        b"website\0" as *const u8 as *const libc::c_char,
        b"https://graphviz.org\0" as *const u8 as *const libc::c_char,
        b"comments\0" as *const u8 as *const libc::c_char,
        b"Visualize and edit graphs of nodes and edges\0" as *const u8
            as *const libc::c_char,
        b"authors\0" as *const u8 as *const libc::c_char,
        authors.as_mut_ptr(),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn load_store_with_attrs(
    mut model: *mut GtkListStore,
    mut job: *mut GVJ_t,
) {
    gtk_list_store_clear(model);
}
#[no_mangle]
pub unsafe extern "C" fn on_drawingarea1_expose_event(
    mut widget: *mut GtkWidget,
    mut event: *mut GdkEventExpose,
    mut user_data: gpointer,
) -> gboolean {
    let mut job: *mut GVJ_t = 0 as *mut GVJ_t;
    let mut cr: *mut cairo_t = 0 as *mut cairo_t;
    job = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"job\0" as *const u8 as *const libc::c_char,
    ) as *mut GVJ_t;
    cr = gdk_cairo_create((*widget).window);
    ((*(*job).callbacks).motion)
        .expect("non-null function pointer")(job, (*job).pointer);
    let ref mut fresh0 = (*job).context;
    *fresh0 = cr as *mut libc::c_void;
    (*job).external_context = 1 as libc::c_int != 0;
    (*job).width = (*widget).allocation.width as libc::c_uint;
    (*job).height = (*widget).allocation.height as libc::c_uint;
    if (*job).has_been_rendered {
        ((*(*job).callbacks).refresh).expect("non-null function pointer")(job);
    } else {
        ((*(*job).callbacks).refresh).expect("non-null function pointer")(job);
    }
    cairo_destroy(cr);
    load_store_with_attrs(
        g_type_check_instance_cast(
            g_object_get_data(
                g_type_check_instance_cast(
                    widget as *mut GTypeInstance,
                    ((20 as libc::c_int) << 2 as libc::c_int) as GType,
                ) as *mut libc::c_void as *mut GObject,
                b"attr_store\0" as *const u8 as *const libc::c_char,
            ) as *mut GTypeInstance,
            gtk_list_store_get_type(),
        ) as *mut libc::c_void as *mut GtkListStore,
        job,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn on_drawingarea1_motion_notify_event(
    mut widget: *mut GtkWidget,
    mut event: *mut GdkEventMotion,
    mut user_data: gpointer,
) -> gboolean {
    let mut job: *mut GVJ_t = 0 as *mut GVJ_t;
    job = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"job\0" as *const u8 as *const libc::c_char,
    ) as *mut GVJ_t;
    (*job).pointer.x = (*event).x;
    (*job).pointer.y = (*event).y;
    gtk_widget_queue_draw(widget);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn on_drawingarea2_motion_notify_event(
    mut widget: *mut GtkWidget,
    mut event: *mut GdkEventMotion,
    mut user_data: gpointer,
) -> gboolean {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn on_drawingarea2_expose_event(
    mut widget: *mut GtkWidget,
    mut event: *mut GdkEventExpose,
    mut user_data: gpointer,
) -> gboolean {
    let mut job: *mut GVJ_t = 0 as *mut GVJ_t;
    let mut cr: *mut cairo_t = 0 as *mut cairo_t;
    let mut tmp: libc::c_double = 0.;
    job = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"job\0" as *const u8 as *const libc::c_char,
    ) as *mut GVJ_t;
    cr = gdk_cairo_create((*widget).window);
    ((*(*job).callbacks).motion)
        .expect("non-null function pointer")(job, (*job).pointer);
    let ref mut fresh1 = (*job).context;
    *fresh1 = cr as *mut libc::c_void;
    (*job).external_context = 1 as libc::c_int != 0;
    (*job).width = (*widget).allocation.width as libc::c_uint;
    (*job).height = (*widget).allocation.height as libc::c_uint;
    tmp = (*job).zoom;
    (*job)
        .zoom = if ((*job).width).wrapping_mul(72 as libc::c_int as libc::c_uint)
        as libc::c_double / ((*job).bb.UR.x * (*job).dpi.x)
        < ((*job).height).wrapping_mul(72 as libc::c_int as libc::c_uint)
            as libc::c_double / ((*job).bb.UR.y * (*job).dpi.y)
    {
        ((*job).width).wrapping_mul(72 as libc::c_int as libc::c_uint) as libc::c_double
            / ((*job).bb.UR.x * (*job).dpi.x)
    } else {
        ((*job).height).wrapping_mul(72 as libc::c_int as libc::c_uint) as libc::c_double
            / ((*job).bb.UR.y * (*job).dpi.y)
    };
    ((*(*job).callbacks).refresh).expect("non-null function pointer")(job);
    (*job).zoom = tmp;
    cairo_destroy(cr);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn on_window1_delete_event(
    mut widget: *mut GtkWidget,
    mut event: *mut GdkEvent,
    mut user_data: gpointer,
) -> gboolean {
    gtk_main_quit();
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn on_drawingarea1_configure_event(
    mut widget: *mut GtkWidget,
    mut event: *mut GdkEventConfigure,
    mut user_data: gpointer,
) -> gboolean {
    let mut job: *mut GVJ_t = 0 as *mut GVJ_t;
    let mut zoom_to_fit: libc::c_double = 0.;
    job = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"job\0" as *const u8 as *const libc::c_char,
    ) as *mut GVJ_t;
    if !(*job).has_been_rendered {
        zoom_to_fit = if ((*event).width as libc::c_double
            / (*job).width as libc::c_double)
            < (*event).height as libc::c_double / (*job).height as libc::c_double
        {
            (*event).width as libc::c_double / (*job).width as libc::c_double
        } else {
            (*event).height as libc::c_double / (*job).height as libc::c_double
        };
        if zoom_to_fit < 1.0f64 {
            (*job).zoom *= zoom_to_fit;
        }
    } else if (*job).fit_mode {
        zoom_to_fit = if ((*event).width as libc::c_double
            / (*job).width as libc::c_double)
            < (*event).height as libc::c_double / (*job).height as libc::c_double
        {
            (*event).width as libc::c_double / (*job).width as libc::c_double
        } else {
            (*event).height as libc::c_double / (*job).height as libc::c_double
        };
        (*job).zoom *= zoom_to_fit;
    }
    if (*event).width as libc::c_uint > (*job).width
        || (*event).height as libc::c_uint > (*job).height
    {
        (*job).has_grown = 1 as libc::c_int != 0;
    }
    (*job).width = (*event).width as libc::c_uint;
    (*job).height = (*event).height as libc::c_uint;
    (*job).needs_refresh = 1 as libc::c_int != 0;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn on_drawingarea1_button_press_event(
    mut widget: *mut GtkWidget,
    mut event: *mut GdkEventButton,
    mut user_data: gpointer,
) -> gboolean {
    let mut job: *mut GVJ_t = 0 as *mut GVJ_t;
    let mut pointer: pointf = pointf { x: 0., y: 0. };
    job = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"job\0" as *const u8 as *const libc::c_char,
    ) as *mut GVJ_t;
    pointer.x = (*event).x;
    pointer.y = (*event).y;
    ((*(*job).callbacks).button_press)
        .expect(
            "non-null function pointer",
        )(job, (*event).button as libc::c_int, pointer);
    load_store_with_attrs(
        g_type_check_instance_cast(
            g_object_get_data(
                g_type_check_instance_cast(
                    widget as *mut GTypeInstance,
                    ((20 as libc::c_int) << 2 as libc::c_int) as GType,
                ) as *mut libc::c_void as *mut GObject,
                b"attr_store\0" as *const u8 as *const libc::c_char,
            ) as *mut GTypeInstance,
            gtk_list_store_get_type(),
        ) as *mut libc::c_void as *mut GtkListStore,
        job,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn on_drawingarea1_button_release_event(
    mut widget: *mut GtkWidget,
    mut event: *mut GdkEventButton,
    mut user_data: gpointer,
) -> gboolean {
    let mut job: *mut GVJ_t = 0 as *mut GVJ_t;
    let mut pointer: pointf = pointf { x: 0., y: 0. };
    job = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"job\0" as *const u8 as *const libc::c_char,
    ) as *mut GVJ_t;
    pointer.x = (*event).x;
    pointer.y = (*event).y;
    ((*(*job).callbacks).button_release)
        .expect(
            "non-null function pointer",
        )(job, (*event).button as libc::c_int, pointer);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn on_drawingarea1_scroll_event(
    mut widget: *mut GtkWidget,
    mut event: *mut GdkEvent,
    mut user_data: gpointer,
) -> gboolean {
    let mut job: *mut GVJ_t = 0 as *mut GVJ_t;
    let mut pointer: pointf = pointf { x: 0., y: 0. };
    job = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"job\0" as *const u8 as *const libc::c_char,
    ) as *mut GVJ_t;
    pointer.x = (*(event as *mut GdkEventScroll)).x;
    pointer.y = (*(event as *mut GdkEventScroll)).y;
    match (*(event as *mut GdkEventScroll)).direction as libc::c_uint {
        0 => {
            ((*(*job).callbacks).button_press)
                .expect("non-null function pointer")(job, 4 as libc::c_int, pointer);
        }
        1 => {
            ((*(*job).callbacks).button_press)
                .expect("non-null function pointer")(job, 5 as libc::c_int, pointer);
        }
        2 | 3 | _ => {}
    }
    gtk_widget_queue_draw(widget);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn on_button1_button_press_event(
    mut widget: *mut GtkWidget,
    mut event: *mut GdkEventButton,
    mut user_data: gpointer,
) -> gboolean {
    fprintf(
        stderr,
        b"will delete selected object\n\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
