use carton_platform::Platform;
use carton_provider::Provider;

#[cfg(target_os = "linux")]
mod platform;
#[cfg(target_os = "linux")]
pub use platform::*;

pub struct X11Platform;

impl Platform for X11Platform {
    fn is_supported(&self) -> bool {
        cfg!(target_os = "linux")
    }

    #[cfg(target_os = "linux")]
    fn get_provider(&self) -> Box<dyn Provider> {
        Box::new(X11Provider::new())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
