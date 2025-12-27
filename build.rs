use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    check_dependencies();

    setup_vulkan();

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

fn setup_vulkan() {
    // Skip Vulkan linking when running in CI or for tests
    let skip_graphics_libs = env::var("CI").is_ok() || env::var("CARGO_CFG_TEST").is_ok();
    
    if skip_graphics_libs {
        println!("cargo:warning=Skipping Vulkan linking (CI or test mode)");
        return;
    }
    
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

fn handle_json_data() {
    // Copy JSON data files to output directory if needed
    let out_dir: PathBuf = PathBuf::from(env::var("OUT_DIR").unwrap());
    let _manifest_dir: PathBuf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Add JSON files that should trigger rebuilds
    println!("cargo:rerun-if-changed=Gods_and_divine_related_entities/");
    println!("cargo:rerun-if-changed=Classes_and_Races/");
    println!("cargo:rerun-if-changed=Items/");
    println!("cargo:rerun-if-changed=Magic/");
    println!("cargo:rerun-if-changed=Skills/");
    println!("cargo:rerun-if-changed=Benefits/");
    println!("cargo:rerun-if-changed=achievements/");

    // Create resources.rs with embedded files if needed
    let resources_path: PathBuf = out_dir.join("resources.rs");
    if !resources_path.exists() {
        std::fs::write(&resources_path, "// Auto-generated resource mappings\n").unwrap();
    }
}

fn setup_linking() {
    // Skip graphics library linking when running in CI or for tests
    // This allows library tests and checks to run without system graphics libraries
    let skip_graphics_libs = env::var("CI").is_ok() || env::var("CARGO_CFG_TEST").is_ok();
    
    // Link additional system libraries as needed
    if cfg!(target_os = "windows") {
        if !skip_graphics_libs {
            println!("cargo:rustc-link-lib=user32");
            println!("cargo:rustc-link-lib=kernel32");
            println!("cargo:rustc-link-lib=gdi32");
            println!("cargo:rustc-link-lib=shell32"); // For file dialogs
            println!("cargo:rustc-link-lib=ole32"); // For COM interfaces
        }
    } else if cfg!(target_os = "linux") {
        if !skip_graphics_libs {
            println!("cargo:rustc-link-lib=X11");
            println!("cargo:rustc-link-lib=Xrandr");
            println!("cargo:rustc-link-lib=Xcursor");
            println!("cargo:rustc-link-lib=Xi"); // XInput for better input support
        }
        // These are always needed even for tests
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=m");
        println!("cargo:rustc-link-lib=stdc++"); // Required for C++ libraries
    } else if cfg!(target_os = "macos") {
        if !skip_graphics_libs {
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
        }
        println!("cargo:rustc-link-lib=c++"); // Required for C++ libraries on macOS
    }
}

fn print_build_info() {
    let target: String = env::var("TARGET").unwrap_or_else(|_| -> String { "unknown".to_string() });
    let profile: String = env::var("PROFILE").unwrap_or_else(|_| -> String { "debug".to_string() });

    println!("cargo:warning=Building for target: {}", target);
    println!("cargo:warning=Build profile: {}", profile);

    // Extra clarity in logs about OS-specific paths used during build
    if cfg!(target_os = "windows") {
        println!("cargo:warning=Detected OS: Windows");
    } else if cfg!(target_os = "macos") {
        println!("cargo:warning=Detected OS: macOS");
    } else if cfg!(target_os = "linux") {
        println!("cargo:warning=Detected OS: Linux");
    } else {
        println!("cargo:warning=Detected OS: Other");
    }
}
