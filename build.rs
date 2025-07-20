use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Build C++ backend files
    build_cpp_backend();
    
    // Setup Vulkan
    setup_vulkan();
    
    // Handle JSON data files
    handle_json_data();
    
    // Setup include paths and linking
    setup_linking();
}

fn build_cpp_backend() {
    let mut build = cc::Build::new();
    
    build
        .cpp(true)
        .std("c++17")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .include("Floor")
        .include("cpp")
        .include("cpp/data")
        // Floor files
        .file("Floor/Floor_1/kruskal.cpp")
        .file("Floor/Floor_1/stats.cpp")
        .file("Floor/Floor_1/time.cpp")
        // Data files
        .file("cpp/data/data.cpp");
    
    // Add debug/release specific flags
    if env::var("PROFILE").unwrap() == "debug" {
        build.flag_if_supported("-g").flag_if_supported("-O0");
    } else {
        build.flag_if_supported("-O3").flag_if_supported("-DNDEBUG");
    }
    
    // Platform-specific configurations
    if cfg!(target_os = "windows") {
        build.flag_if_supported("/std:c++17");
    }
    
    build.compile("dungeon_crawler_backend");
    
    println!("cargo:rerun-if-changed=Floor/");
    println!("cargo:rerun-if-changed=cpp/");
}

fn setup_vulkan() {
    // Find Vulkan SDK
    if let Ok(vulkan_sdk) = env::var("VULKAN_SDK") {
        println!("cargo:rustc-link-search=native={}/Lib", vulkan_sdk);
        println!("cargo:rustc-link-lib=vulkan-1");
        println!("cargo:include={}/Include", vulkan_sdk);
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=vulkan");
        pkg_config::probe("vulkan").unwrap();
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");
    }
    
    println!("cargo:rerun-if-env-changed=VULKAN_SDK");
}

fn handle_json_data() {
    // Copy JSON data files to output directory if needed
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    
    // Add JSON files that should trigger rebuilds
    println!("cargo:rerun-if-changed=data/");
    println!("cargo:rerun-if-changed=assets/");
    
    // If you need to process JSON at build time, add logic here
    // For example, validating JSON schemas or converting formats
}

fn setup_linking() {
    // Link additional system libraries as needed
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=kernel32");
        println!("cargo:rustc-link-lib=gdi32");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=Xrandr");
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=m");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
    }
}
