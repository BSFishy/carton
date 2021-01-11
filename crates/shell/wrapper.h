#if USE_WINDOWS
#  error "Windows is not supported yet"
#elif USE_MACOS
#  error "MacOS is not supported yet"
#elif USE_XCB
#  include <xcb/xcb.h>
#elif USE_WAYLAND
#  include <wayland-client.h>
#elif USE_OPENGL
#  include <GL/gl.h>
#elif USE_VULKAN
#  error "Vulkan is not supported yet"
#else
#  error "Unsupported platform"
#endif
