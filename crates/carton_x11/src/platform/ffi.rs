use std::ffi::{c_void};
use std::os::raw::{c_char, c_int, c_uint};
use std::intrinsics::transmute;

pub const X_PROTOCOL: i32 = 11;
pub const X_PROTOCOL_REVISION: i32 = 0;
pub const X_TCP_PORT: i32 = 6000;
pub const XCB_NONE: u8 = 0;
pub const XCB_COPY_FROM_PARENT: u8 = 0;
pub const XCB_CURRENT_TIME: u8 = 0;
pub const XCB_NO_SYMBOL: u8 = 0;

pub type XCBConnection = *mut c_void;
pub type XCBWindow = u32;

pub type XCBDrawable = u32;

pub type XCBColormap = u32;
pub type XCBVisualId = u32;

pub type XCBKeycode = u8;

pub type XCBAtom = u32;

pub enum XCBError {
    ConnError = 1,
    ConnClosedExtNotSupported = 2,
    ConnClosedMemInsufficient = 3,
    ConnClosedReqLenExceed = 4,
    ConnClosedParseErr = 5,
    ConnClosedInvalidScreen = 6,
    ConnClosedFDPassingFailed = 7,
}

pub enum XCBWindowClass {
    CopyFromParent = 0,
    InputOutput = 1,
    InputOnly = 2,
}

pub enum XCBPropMode {
    Replace = 0,
    Prepend = 1,
    Append = 2,
}

pub enum XCBAtomEnum {
    // XCB_ATOM_NONE = 0,
    // XCB_ATOM_ANY = 0,
    Primary = 1,
    Secondary = 2,
    Arc = 3,
    Atom = 4,
    Bitmap = 5,
    Cardinal = 6,
    Colormap = 7,
    Cursor = 8,
    CutBuffer0 = 9,
    CutBuffer1 = 10,
    CutBuffer2 = 11,
    CutBuffer3 = 12,
    CutBuffer4 = 13,
    CutBuffer5 = 14,
    CutBuffer6 = 15,
    CutBuffer7 = 16,
    Drawable = 17,
    Font = 18,
    Integer = 19,
    Pixmap = 20,
    Point = 21,
    Rectangle = 22,
    ResourceManager = 23,
    RgbColorMap = 24,
    RgbBestMap = 25,
    RgbBlueMap = 26,
    RgbDefaultMap = 27,
    RgbGrayMap = 28,
    RgbGreenMap = 29,
    RgbRedMap = 30,
    String = 31,
    VisualID = 32,
    Window = 33,
    WmCommand = 34,
    WmHints = 35,
    WmClientMachine = 36,
    WmIconName = 37,
    WmIconSize = 38,
    WmName = 39,
    WmNormalHints = 40,
    WmSizeHints = 41,
    WmZoomHints = 42,
    MinSpace = 43,
    NormSpace = 44,
    MaxSpace = 45,
    EndSpace = 46,
    SuperscriptX = 47,
    SuperscriptY = 48,
    SubscriptX = 49,
    SubscriptY = 50,
    UnderlinePosition = 51,
    UnderlineThickness = 52,
    StrikeoutAscent = 53,
    StrikeoutDescent = 54,
    ItalicAngle = 55,
    XHeight = 56,
    QuadWidth = 57,
    Weight = 58,
    PointSize = 59,
    Resolution = 60,
    Copyright = 61,
    Notice = 62,
    FontName = 63,
    FamilyName = 64,
    FullName = 65,
    CapHeight = 66,
    WmClass = 67,
    WmTransientFor = 68,
}

pub enum XCBConfigWindow {
    X = 1,
    Y = 2,
    Width = 4,
    Height = 8,
    BorderWidth = 16,
    Sibling = 32,
    StackMode = 64,
}

pub enum XCBEventMask {
    NoEvent = 0,
    KeyPress = 1,
    KeyRelease = 2,
    ButtonPress = 4,
    ButtonRelease = 8,
    EnterWindow = 16,
    LeaveWindow = 32,
    PointerMotion = 64,
    PointerMotionHint = 128,
    Button1Motion = 256,
    Button2Motion = 512,
    Button3Motion = 1024,
    Button4Motion = 2048,
    Button5Motion = 4096,
    ButtonMotion = 8192,
    KeymapState = 16384,
    Exposure = 32768,
    VisibilityChange = 65536,
    StructureNotify = 131072,
    ResizeRedirect = 262144,
    SubstructureNotify = 524288,
    SubstructureRedirect = 1048576,
    FocusChange = 2097152,
    PropertyChange = 4194304,
    ColorMapChange = 8388608,
    OwnerGrabButton = 16777216,
}

#[repr(u32)]
pub enum XCBCW {
    BackPixmap = 1,
    BackPixel = 2,
    BorderPixmap = 4,
    BorderPixel = 8,
    BitGravity = 16,
    WinGravity = 32,
    BackingStore = 64,
    BackingPlanes = 128,
    BackingPixel = 256,
    OverrideRedirect = 512,
    SaveUnder = 1024,
    EventMask = 2048,
    DontPropagate = 4096,
    Colormap = 8192,
    Cursor = 16384
}

#[repr(u8)]
pub enum XCBEventType {
    KeyPress = 2,
    KeyRelease = 3,
    ButtonPress = 4,
    ButtonRelease = 5,
    MotionNotify = 6,
    EnterNotify = 7,
    LeaveNotify = 8,
    FocusIn = 9,
    FocusOut = 10,
    KeymapNotify = 11,
    Expose = 12,
    GraphicsExposure = 13,
    NoExposure = 14,
    VisibilityNotify = 15,
    CreateNotify = 16,
    DestroyNotify = 17,
    UnmapNotify = 18,
    MapNotify = 19,
    MapRequest = 20,
    ReparentNotify = 21,
    ConfigureNotify = 22,
    ConfigureRequest = 23,
    GravityNotify = 24,
    ResizeRequest = 25,
    CirculateNotify = 26,
    CirculateRequest = 27,
    PropertyNotify = 28,
    SelectionClear = 29,
    SelectionRequest = 30,
    SelectionNotify = 31,
    ColormapNotify = 32,
    ClientMessage = 33,
    MappingNotify = 34,
    GEGeneric = 35,
}

impl From<u8> for XCBEventType {
    fn from(other: u8) -> Self {
        unsafe {
            transmute(other)
        }
    }
}

#[repr(C)]
pub struct XCBConfigureNotifyEvent {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: XCBWindow,
    pub window: XCBWindow,
    pub above_sibling: XCBWindow,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: u8,
    pub pad1: u8,
}

#[repr(C)]
pub struct XCBGenericError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub resource_id: u32,
    pub minor_code: u16,
    pub major_code: u8,
    pub pad0: u8,
    pub pad: [u32; 5],
    pub full_sequence: u32,
}

#[repr(C)]
pub struct XCBSetup {
    pub status: u8,
    pub pad0: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub vendor_len: u16,
    pub maximum_request_length: u16,
    pub roots_len: u8,
    pub pixmap_formats_len: u8,
    pub image_byte_order: u8,
    pub bitmap_format_bit_order: u8,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: XCBKeycode,
    pub max_keycode: XCBKeycode,
    pub pad1: [u8; 4],
}

#[repr(C)]
pub struct XCBScreen {
    pub root: XCBWindow,
    pub default_colormap: XCBColormap,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: XCBVisualId,
    pub backing_stores: u8,
    pub save_unders: u8,
    pub root_depth: u8,
    pub allowed_depths_len: u8,
}

#[repr(C)]
pub struct XCBScreenIterator {
    pub data: *mut XCBScreen,
    pub rem: c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct XCBVoidCookie {
    pub sequence: c_uint,
}

#[repr(C)]
pub struct XCBGetGeometryCookie {
    pub sequence: c_uint,
}

#[repr(C)]
pub struct XCBGetGeometryReply {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: XCBWindow,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub pad8: [u8; 2],
}

#[repr(C)]
pub struct XCBGenericEvent {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub pad: [u32; 7],
    pub full_sequence: u32,
}

#[link(name = "xcb")]
extern "C" {
    pub fn xcb_connect(displayname: *const c_char, screenp: *mut c_int) -> *mut XCBConnection;

    // xcb_generic_error_t *xcb_request_check(xcb_connection_t *c, xcb_void_cookie_t cookie);
    pub fn xcb_request_check(c: *mut XCBConnection, cookie: XCBVoidCookie) -> *mut XCBGenericError;

    pub fn xcb_get_setup(connection: *mut XCBConnection) -> *const XCBSetup;

    pub fn xcb_setup_roots_iterator(r: *const XCBSetup) -> XCBScreenIterator;

    pub fn xcb_generate_id(connection: *mut XCBConnection) -> XCBWindow;

    pub fn xcb_create_window(connection: *mut XCBConnection,
                             depth: u8,
                             wid: XCBWindow,
                             parent: XCBWindow,
                             x: i16,
                             y: i16,
                             width: u16,
                             height: u16,
                             border_width: u16,
                             _class: u16,
                             visual: XCBVisualId,
                             value_mask: u32,
                             value_list: *const c_void) -> XCBVoidCookie;

    pub fn xcb_map_window(c: *mut XCBConnection, window: XCBWindow) -> XCBVoidCookie;

    pub fn xcb_unmap_window(c: *mut XCBConnection, window: XCBWindow) -> XCBVoidCookie;

    pub fn xcb_change_property(c: *mut XCBConnection, mode: u8, window: XCBWindow, property: XCBAtom, ty: XCBAtom, format: u8, data_len: u32, data: *const c_void) -> XCBVoidCookie;

    pub fn xcb_configure_window(c: *mut XCBConnection, window: XCBWindow, mask: u16, value_list: *const c_void) -> XCBVoidCookie;

    pub fn xcb_get_geometry(c: *mut XCBConnection, drawable: XCBDrawable) -> XCBGetGeometryCookie;

    pub fn xcb_get_geometry_reply(c: *mut XCBConnection, cookie: XCBGetGeometryCookie, e: *mut *mut XCBGenericError) -> *mut XCBGetGeometryReply;

    // xcb_generic_event_t *xcb_wait_for_event(xcb_connection_t *c);
    pub fn xcb_wait_for_event(c: *mut XCBConnection) -> *mut XCBGenericEvent;

    pub fn xcb_flush(c: *mut XCBConnection) -> c_int;

    pub fn xcb_disconnect(c: *mut XCBConnection);
}
