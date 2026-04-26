use crate::wrap::{
    colors::wrap_default_colors, enums::wrap_exposed_enums, raylib_api::RayLibApiDefinition,
};

mod bind;
mod wrap;

pub fn main() {
    // Files to watch that should trigger a rebuild
    println!("cargo:rerun-if-changed=src/wrapper.h");

    // Compile raylib
    bind::compile_raylib("third_party/raylib");

    // Link libraries
    bind::link_libs();

    // Generate bindings
    bind::generate_bindings("src/wrapper.h");

    // Load the API definitions
    let api_defs =
        RayLibApiDefinition::load("third_party/raylib/tools/rlparser/output/raylib_api.json")
            .unwrap();

    // Generate safe wrappers
    wrap_exposed_enums(api_defs.clone());
    wrap_default_colors(api_defs);
}
