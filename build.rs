use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    check_dependencies();

    build_cpp_backend();

    setup_vulkan();

    // setup_bullet_physics();

    handle_json_data();

    setup_linking();

    print_build_info();
}

fn check_dependencies() {
    // Check for required tools
    println!("cargo:warning=Checking for required build dependencies...");

    // For packaging, these would be needed but not required for the build itself
    if std::env::var("CARGO_FEATURE_PACKAGE").is_ok() {
        // TODO: IDK if this actually works so please check that it does
        if cfg!(target_os = "windows") {
            // Check for WiX Toolset (for MSI creation)
            let wix: bool = Command::new("where").arg("candle").output().is_ok();
            if !wix {
                println!("cargo:warning=WiX Toolset not found. MSI creation may fail.");
            }
        } else if cfg!(target_os = "macos") {
            // Check for pkgbuild (for DMG creation)
            let pkgbuild: bool = Command::new("which").arg("pkgbuild").output().is_ok();
            if !pkgbuild {
                println!("cargo:warning=pkgbuild not found. DMG creation may fail.");
            }
        }
    }
}

fn build_cpp_backend() {
    let mut build: cc::Build = cc::Build::new();

    // Common configuration
    build
        .cpp(true)
        .std("c++17")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .include("Floor")
        .include("cpp")
        .include("cpp/data")
        .include("cpp/include") // Add include directory for headers
        // Floor files
        .file("Floor/Floor_1/kruskal.cpp")
        .file("Floor/Floor_1/stats.cpp")
        .file("Floor/Floor_1/time.cpp")
        // Data files
        .file("cpp/data/json.cpp");
    // .file("cpp/data/save.cpp");

    // Find and add all other .cpp files
    if let Ok(entries) = std::fs::read_dir("cpp") {
        for entry in entries.filter_map(Result::ok) {
            let path: PathBuf = entry.path();
            if path.is_file()
                && path
                    .extension()
                    .map_or(false, |ext: &std::ffi::OsStr| ext == "cpp")
            {
                build.file(path);
            }
        }
    }

    // Add debug/release specific flags
    if env::var("PROFILE").unwrap() == "debug" {
        build.flag_if_supported("-g").flag_if_supported("-O0");
    } else {
        build.flag_if_supported("-O3").flag_if_supported("-DNDEBUG");
    }

    // Platform-specific configurations
    if cfg!(target_os = "windows") {
        build.flag_if_supported("/std:c++17");
        build.flag_if_supported("/EHsc"); // Exception handling
    } else if cfg!(target_os = "macos") {
        // Ensure compatibility with Apple Silicon
        if env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() == "aarch64" {
            build.flag("-arch").flag("arm64");
        }
    }

    build.compile("dungeon_crawler_backend");

    println!("cargo:rerun-if-changed=Floor/");
    println!("cargo:rerun-if-changed=cpp/");
}

fn setup_vulkan() {
    // Find Vulkan SDK
    if let Ok(vulkan_sdk) = env::var("VULKAN_SDK") {
        // Standard Vulkan setup
        println!("cargo:rustc-link-search=native={}/Lib", vulkan_sdk);
        println!("cargo:rustc-link-lib=vulkan-1");
        println!("cargo:include={}/Include", vulkan_sdk);
    } else if cfg!(target_os = "linux") {
        // Linux fallback using pkg-config
        if Command::new("pkg-config")
            .arg("--exists")
            .arg("vulkan")
            .status()
            .is_ok()
        {
            println!("cargo:rustc-link-lib=vulkan");
        } else {
            println!("cargo:warning=Vulkan SDK not found. Install vulkan-headers and vulkan-loader packages.");
        }
    } else if cfg!(target_os = "macos") {
        // macOS uses Metal via MoltenVK
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");
        println!("cargo:rustc-link-lib=framework=QuartzCore"); // For CAMetalLayer

        // Check for MoltenVK
        if let Ok(home) = env::var("HOME") {
            let moltenvk_path: String = format!("{}/MoltenVK/dylib", home);
            if std::path::Path::new(&moltenvk_path).exists() {
                println!("cargo:rustc-link-search=native={}", moltenvk_path);
                println!("cargo:rustc-link-lib=MoltenVK");
            } else {
                println!("cargo:warning=MoltenVK not found in ~/MoltenVK/dylib. Vulkan support may be limited.");
            }
        }
    }

    println!("cargo:rerun-if-env-changed=VULKAN_SDK");
}

// We don't use bullet for anything yet so for the sake of easy building we can just not use it in the build.rs file
fn setup_bullet_physics() {
    // Link Bullet Physics C++ libraries
    // On Windows, set BULLET_DIR env variable to Bullet install path, or ensure Bullet libs are in your library path
    if let Ok(bullet_dir) = env::var("BULLET_DIR") {
        println!("cargo:rustc-link-search=native={}/lib", bullet_dir);
        println!("cargo:include={}/include/bullet", bullet_dir);
        println!("cargo:rustc-link-lib=BulletDynamics");
        println!("cargo:rustc-link-lib=BulletCollision");
        println!("cargo:rustc-link-lib=LinearMath");
    } else {
        // Try to find Bullet via pkg-config on Linux/macOS
        if !cfg!(target_os = "windows")
            && Command::new("pkg-config")
                .arg("--exists")
                .arg("bullet")
                .status()
                .is_ok()
        {
            let output: std::process::Output = Command::new("pkg-config")
                .arg("--libs")
                .arg("bullet")
                .output()
                .expect("Failed to execute pkg-config");

            let libs: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
            for lib in libs.split_whitespace() {
                if lib.starts_with("-l") {
                    println!("cargo:rustc-link-lib={}", &lib[2..]);
                }
            }
        } else {
            // Default Bullet libraries to link
            println!("cargo:rustc-link-lib=BulletDynamics");
            println!("cargo:rustc-link-lib=BulletCollision");
            println!("cargo:rustc-link-lib=LinearMath");
        }
    }

    println!("cargo:rerun-if-env-changed=BULLET_DIR");
}

fn handle_json_data() {
    // Copy JSON data files to output directory if needed
    let out_dir: PathBuf = PathBuf::from(env::var("OUT_DIR").unwrap());
    let _manifest_dir: PathBuf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()); // manifest_dir is Unused

    // Add JSON files that should trigger rebuilds
    println!("cargo:rerun-if-changed=data/");
    println!("cargo:rerun-if-changed=assets/");
    println!("cargo:rerun-if-changed=Scheme/");
    println!("cargo:rerun-if-changed=Gods_and_divine_related_entities/");
    println!("cargo:rerun-if-changed=Classes_and_Races/");
    println!("cargo:rerun-if-changed=Items/");
    println!("cargo:rerun-if-changed=Magic/");
    println!("cargo:rerun-if-changed=Skills/");
    println!("cargo:rerun-if-changed=Benefits/");

    // Create resources.rs with embedded files if needed
    let resources_path: PathBuf = out_dir.join("resources.rs");
    if !resources_path.exists() {
        std::fs::write(&resources_path, "// Auto-generated resource mappings\n").unwrap();
    }
}

fn setup_linking() {
    // Link additional system libraries as needed
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=kernel32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=shell32"); // For file dialogs
        println!("cargo:rustc-link-lib=ole32"); // For COM interfaces
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=Xrandr");
        println!("cargo:rustc-link-lib=Xcursor");
        println!("cargo:rustc-link-lib=Xi"); // XInput for better input support
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=m");
        println!("cargo:rustc-link-lib=stdc++"); // Required for C++ libraries
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=c++"); // Required for C++ libraries on macOS
    }
}

fn print_build_info() {
    let target: String = env::var("TARGET").unwrap_or_else(|_| -> String { "unknown".to_string() });
    let profile: String = env::var("PROFILE").unwrap_or_else(|_| -> String { "debug".to_string() });

    println!("cargo:warning=Building for target: {}", target);
    println!("cargo:warning=Build profile: {}", profile);
}
