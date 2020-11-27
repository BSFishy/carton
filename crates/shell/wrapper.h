#if USE_WINDOWS
#  error "Windows is not supported yet"
#elif USE_MACOS
#  error "MacOS is not supported yet"
#elif USE_XCB
#  include <xcb/xcb.h>
#else
#  error "Unsupported platform"
#endif
