// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use cmake::Config;

fn main() {
    // Skip building the library when building documentation
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();

    println!("{}", out_dir);

    unsafe { std::env::set_var("CMAKE_GENERATOR", "Ninja") };
    unsafe { std::env::set_var("CMAKE_BUILD_TYPE", "Release") };

    let cxxflags = if cfg!(windows) { 
        "/EHsc" 
    } else { 
        "-Wno-stringop-overflow" 
    };

    // Determine optimization level from features
    let opt_flag = if cfg!(feature = "opt-o0") {
        if cfg!(windows) { "/Od" } else { "-O0" }
    } else if cfg!(feature = "opt-o1") {
        if cfg!(windows) { "/O1" } else { "-O1" }
    } else if cfg!(feature = "opt-o3") {
        if cfg!(windows) { "/O2" } else { "-O3" }  // MSVC /O2 is roughly equivalent to GCC -O3
    } else if cfg!(feature = "opt-os") {
        if cfg!(windows) { "/Os" } else { "-Os" }
    } else if cfg!(feature = "opt-ofast") {
        if cfg!(windows) { "/O2 /fp:fast" } else { "-Ofast" }
    } else {
        // Default to O2
        if cfg!(windows) { "/O2" } else { "-O2" }
    };

    // Build the C++ library using CMake
    Config::new("boolop_wrapper")
        .cxxflag(cxxflags)
        .cxxflag(opt_flag)
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_SUPPRESS_DEVELOPER_WARNINGS", "ON")
        .build();

    // Build the Rust bridge
    cxx_build::bridge("src/lib.rs")
        .std("c++20")  // Match CMakeLists.txt C++20 standard
        .file("src/booleans_rs.cpp")
        .include("./src")
        .include("./boolop_wrapper")
        .include(format!("{out_dir}/include"))
        .compile("booleans_rs");

     // Link the static libraries in correct order
     println!("cargo:rustc-link-search={out_dir}/build");
     println!("cargo:rustc-link-search={out_dir}/build/shewchuk_predicates");
     println!("cargo:rustc-link-search={out_dir}/lib");
 
     // Link main library first
     println!("cargo:rustc-link-lib=static=boolop_wrapper");
     
     // Link predicates library
     println!("cargo:rustc-link-lib=static=shewchuk_predicates");
     
     // Link TBB libraries as static to avoid runtime issues
     println!("cargo:rustc-link-lib=static=tbb");
     println!("cargo:rustc-link-lib=static=tbbmalloc");
 
     // Link system libraries
     if cfg!(unix) {
         println!("cargo:rustc-link-lib=m");
         println!("cargo:rustc-link-lib=pthread");
         println!("cargo:rustc-link-lib=dl");
     }
 
     // For C++ standard library
     if cfg!(target_os = "linux") {
         println!("cargo:rustc-link-lib=stdc++");
     } else if cfg!(target_os = "macos") {
         println!("cargo:rustc-link-lib=c++");
     }
 

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/booleans_rs.hpp");
    println!("cargo:rerun-if-changed=src/booleans_rs.cpp");
}
