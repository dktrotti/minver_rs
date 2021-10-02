use minver_rs::{build_utils, MinverConfig};

fn main() {
    let config = MinverConfig::read_from_env().expect("Failed to parse configuration");
    if let Err(e) = simple_logger::init_with_level(config.log_level) {
        println!("Failed to initialize log: {}", e);
    }
    build_utils::default_build_action_silent(&config);
}
