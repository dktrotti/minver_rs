use log::Level;
use minver_rs::build_utils;

fn main() {
    if let Err(e) = simple_logger::init_with_level(Level::Warn) {
        println!("Failed to initialize log: {}", e);
    }
    build_utils::default_build_action_silent();
}
