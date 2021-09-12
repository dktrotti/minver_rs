use minver_rs::build_utils;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    build_utils::default_build_action();
}
