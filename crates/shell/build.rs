#![allow(clippy::if_same_then_else)]

use std::env;
use std::path::PathBuf;

macro_rules! cargo {
    ($value:expr) => {
        println!("cargo:{}", $value)
    }
}

macro_rules! warning {
    ($message:expr) => {
        cargo!(format!("warning={}", $message));
    };
    ($message:expr $(, $extra:expr)*) => {
        warning!(format!($message $(, $extra)*));
    }
}

macro_rules! config {
    ($key:expr) => {
        cargo!(format!("rustc-cfg={}", $key));
    };
    ($key:expr => $value:expr) => {
        config!(format!("{}=\"{}\"", $key, $value));
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Platform {
    Windows,
    Mac,

    X11,
    Wayland,

    OpenGL,
    Vulkan,
}

/// TODO: improve error messages on panics and whatnot to give more details on how to fix the errors
fn main() {
    let mut platforms: Vec<Platform> = Vec::new();

    add_platforms(&mut platforms);
    validate_platforms(&mut platforms);
    let platform = finalize_platform(platforms);

    link_with_platform(&platform);

    config!("platform" => match platform {
        Platform::Windows => "windows",
        Platform::Mac => "macos",
        Platform::X11 => "x11",
        Platform::Wayland => "wayland",
        Platform::OpenGL => "opengl",
        Platform::Vulkan => "vulkan",
    });

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Add the platform definition
        .clang_arg(format!("-D{}", get_clang_definition(&platform)))
        // Use Rust's core instead of libstd
        .use_core()
        // Make the default type of enums use Rust style
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    if platform == Platform::Wayland {
        bindings = bindings
            .blacklist_item("FP_NAN")
            .blacklist_item("FP_INFINITE")
            .blacklist_item("FP_ZERO")
            .blacklist_item("FP_SUBNORMAL")
            .blacklist_item("FP_NORMAL");
    }

    let bindings = bindings
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn add_platforms(platforms: &mut Vec<Platform>) {
    if cfg!(feature = "windows") {
        platforms.push(Platform::Windows)
    }

    if cfg!(feature = "macos") {
        platforms.push(Platform::Mac)
    }

    if cfg!(feature = "x11") {
        platforms.push(Platform::X11)
    }

    if cfg!(feature = "wayland") {
        platforms.push(Platform::Wayland)
    }

    if cfg!(feature = "opengl") {
        platforms.push(Platform::OpenGL)
    }

    if cfg!(feature = "vulkan") {
        platforms.push(Platform::Vulkan)
    }
}

fn validate_platforms(platforms: &mut Vec<Platform>) {
    if platforms.contains(&Platform::X11) && platforms.contains(&Platform::Wayland) {
        panic!("X11 and Wayland are mutually exclusive")
    }
}

fn finalize_platform(platforms: Vec<Platform>) -> Platform {
    let mut platform: Option<Platform> = None;

    if platforms.contains(&Platform::Vulkan) {
        platform = Some(Platform::Vulkan)
    }

    if platforms.contains(&Platform::OpenGL) {
        platform = Some(Platform::OpenGL);
    }

    if platform.is_none() {
        if cfg!(target_os = "windows") {
            if platforms.contains(&Platform::Windows) {
                platform = Some(Platform::Windows)
            }
        } else if cfg!(target_os = "macos") {
            if platforms.contains(&Platform::Mac) {
                platform = Some(Platform::Mac)
            }
        } else if cfg!(target_os = "linux") {
            if platforms.contains(&Platform::X11) {
                platform = Some(Platform::X11)
            }

            if platforms.contains(&Platform::Wayland) {
                platform = Some(Platform::Wayland)
            }
        }
    }

    if platform.is_none() {
        if cfg!(target_os = "windows") {
            platform = Some(Platform::Windows)
        } else if cfg!(target_os = "macos") {
            platform = Some(Platform::Mac)
        } else if cfg!(target_os = "linux") {
            // TODO: detect if X11 or Wayland are available first
            platform = Some(Platform::X11)
        }

        if let Some(platform) = platform {
            warning!("No suitable platform specified. Defaulting to {:?}.", platform)
        }
    }

    platform.expect("Could not find a suitable platform")
}

fn link_with_platform(platform: &Platform) {
    match platform {
        Platform::X11 => {
            pkg_config::Config::new()
                .cargo_metadata(true)
                .atleast_version("1.14")
                .probe("xcb")
                .expect("Unable to find XCB");
        }
        Platform::Wayland => {
            pkg_config::Config::new()
                .cargo_metadata(true)
                .atleast_version("1.18")
                .probe("wayland-client")
                .expect("Unable to find Wayland Client library");
        }
        Platform::OpenGL => {
            if cfg!(target_os = "linux") {
                pkg_config::Config::new()
                    .cargo_metadata(true)
                    .atleast_version("4.5")
                    .probe("opengl")
                    .expect("Unable to find OpenGL");
            } else {
                unimplemented!()
            }
        }
        _ => unimplemented!(),
    }
}

fn get_clang_definition(platform: &Platform) -> &'static str {
    match platform {
        Platform::Windows => "USE_WINDOWS",
        Platform::Mac => "USE_MACOS",
        Platform::X11 => "USE_XCB",
        Platform::Wayland => "USE_WAYLAND",
        Platform::OpenGL => "USE_OPENGL",
        Platform::Vulkan => "USE_VULKAN",
    }
}
