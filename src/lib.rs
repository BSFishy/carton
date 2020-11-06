pub use carton_platform::Platform;
pub use carton_provider::Provider;

pub use carton_view::View;
pub use carton_window::Window;

pub mod error;

pub struct Carton;

static mut PLATFORMS: Vec<Box<dyn Platform>> = vec![];

impl Carton {
    pub fn new_window<T: Window>() -> Result<T, &'static str> {
        Err("")
    }

    pub unsafe fn add_platform(platform: Box<dyn Platform>) {
        PLATFORMS.insert(0, platform)
    }

    pub fn add_default_platforms() {
        // TODO(BSFishy): add default platforms here
    }

    pub unsafe fn get_current_platform<'a>() -> Result<&'a Box<dyn Platform>, error::NoPlatformError> {
        for platform in PLATFORMS.iter() {
            if platform.is_supported() {
                return Ok(platform);
            }
        }

        Err(error::NoPlatformError)
    }
}
