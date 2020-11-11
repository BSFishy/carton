use carton_x11::{X11Provider};
use carton_common::{Size, Point};
use carton_provider::Provider;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let provider = X11Provider::new();
    let mut window = provider.create_window();
    window.set_title(String::from("This is a test"));
    window.set_size(Size::new(500, 500));
    window.set_position(Point::new(500f32, 0f32));
    window.show();
    // sleep(Duration::from_secs(5));
    println!("Size: {}, position: {}", window.get_size(), window.get_position());
    window.run();
}
