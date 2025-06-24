fn main() {
    // Template for compiling C++ files with Rust using the cc crate.
    cc::Build::new()
        .cpp(true)
        .file("Floor/Floor_1/kruskal.cpp")
        .file("Floor/Floor_1/stats.cpp")
        .file("Floor/Floor_1/time.cpp")
        .compile("floor_cpp");
    // Add more files as needed above.
}