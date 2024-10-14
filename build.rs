use ispc::TargetISA;

fn add_offsetallocator() {
    // using bindgen, generate binding code
    bindgen::Builder::default()
        //.clang_arg("-x c++")
        .clang_args(["-x", "c++", "-std=c++20"])
        .header("c/offsetallocator/offsetAllocator.hpp")
        .opaque_type("std::.*")
        .opaque_type("Allocator")
        .allowlist_type("StorageReport")
        .allowlist_type("StorageReportFull")
        .allowlist_type("Allocation")
        .allowlist_function("Allocator_.*")
        .allowlist_function("Allocator_New")
        .allowlist_function("Allocator_Free")
        .allowlist_item("Allocator.*")
        .allowlist_var("Allocator.*")
        .generate().unwrap()
        .write_to_file("src/offsetallocator.rs").unwrap();

// using cc, build and link c code
    cc::Build::new()
        .std("c++20")
        .out_dir("obj")
        .file("c/offsetallocator/offsetAllocator.cpp")
        .cpp(true)
        //.cpp_link_stdlib("stdc++")
        .compile("offsetallocator");

// csbindgen code, generate both rust ffi and C# dll import
    csbindgen::Builder::default()
        .input_bindgen_file("src/offsetallocator.rs")            // read from bindgen generated code
        .rust_file_header("use super::offsetallocator::*;")     // import bindgen generated modules(struct/method)
        .csharp_entry_point_prefix("csbindgen_")
        .csharp_dll_name("primrose_rust")
        .csharp_dll_name_if("PRIMROSE_IOS", "__Internal")
        .csharp_namespace("Primrose.Native.offsetallocator")
        .csharp_entry_point_prefix("csbindgen_")
        .csharp_class_name("NativeMethods")
        .generate_to_file("src/offsetallocator_ffi.rs", "../PrimroseEngine/Engine/Source/Native/Native_offsetallocator.g.cs")
        .unwrap();
}

fn add_pitchtracker()
{
    // using bindgen, generate binding code
    bindgen::Builder::default()
        //.clang_arg("-x c++")
        .clang_args(["-x", "c++"])
        .header("c/pitchtracker/PitchWrapper.h")
        .opaque_type("std::.*")
        .allowlist_type("Analyzer")
        .allowlist_type("PtAKF")
        .allowlist_type("PtDyWa")
        .allowlist_function("Analyzer_.*")
        .allowlist_function("PtAKF_.*")
        .allowlist_function("PtDyWa_.*")
        .blocklist_item("std::value")
        .blocklist_item("__gnu_cxx::__min")
        .blocklist_item("__gnu_cxx::__max")
        .generate().unwrap()
        .write_to_file("src/pitchtracker.rs").unwrap();

// using cc, build and link c code
    cc::Build::new()
        .out_dir("obj")
        .file("c/pitchtracker/PitchWrapper.cpp")
        .file("c/pitchtracker/Helper.cpp")
        .file("c/pitchtracker/ptAKF.cpp")
        .file("c/pitchtracker/performous/pitch.cc")
        .file("c/pitchtracker/FFT/FFT.cpp")
        .file("c/pitchtracker/FFT/RealFFTf.cpp")
        .file("c/pitchtracker/dywapitchtrack/ptDyWa.cpp")
        .file("c/pitchtracker/dywapitchtrack/dywapitchtrack.cpp")
        .cpp(true)
        //.cpp_link_stdlib("stdc++")
        .compile("pitchtracker");

// csbindgen code, generate both rust ffi and C# dll import
    csbindgen::Builder::default()
        .input_bindgen_file("src/pitchtracker.rs")            // read from bindgen generated code
        .rust_file_header("use super::pitchtracker::*;")     // import bindgen generated modules(struct/method)
        .csharp_entry_point_prefix("csbindgen_")
        .csharp_dll_name("primrose_rust")
        .csharp_dll_name_if("PRIMROSE_IOS", "__Internal")
        .csharp_namespace("Primrose.Native.pitchtracker")
        .csharp_entry_point_prefix("csbindgen_")
        .csharp_class_name("NativeMethods")
        .generate_to_file("src/pitchtracker_ffi.rs", "../PrimroseEngine/Engine/Source/Native/Native_pitchtracker.g.cs")
        .unwrap();
}


fn add_ozz_cpp()
{
    /*
    // using bindgen, generate binding code
    bindgen::Builder::default()
        //.clang_arg("-x c++")
        .clang_args(["-x", "c++"])
        //.header("c/ozz-animation/include/**/*.h")
        .clang_arg("-Ic/ozz-animation/include")
        .header("c/ozz-animation/include/ozz/base/span.h")
        .header("c/ozz-animation/include/ozz/base/containers/vector.h")
        .header("c/ozz-animation/include/ozz/animation/offline/animation_builder.h")
        .header("c/ozz-animation/include/ozz/animation/offline/animation_optimizer.h")
        .header("c/ozz-animation/include/ozz/animation/offline/additive_animation_builder.h")
        .header("c/ozz-animation/include/ozz/animation/offline/raw_animation.h")
        .header("c/ozz-animation/include/ozz/animation/offline/raw_animation_utils.h")
        .header("c/ozz-animation/include/ozz/animation/offline/raw_skeleton.h")
        .header("c/ozz-animation/include/ozz/animation/offline/raw_track.h")
        .header("c/ozz-animation/include/ozz/animation/offline/skeleton_builder.h")
        .header("c/ozz-animation/include/ozz/animation/offline/track_builder.h")
        .header("c/ozz-animation/include/ozz/animation/offline/track_optimizer.h")
        .allowlist_type("ozz::.*")
        //.blocklist_function("_.*")
        //.blocklist_function("std.*")
        //.blocklist_type("std.*")
        .layout_tests(false)
        .opaque_type("std::.*")
        .generate().unwrap()
        .write_to_file("src/ozz_cpp.rs").unwrap();

    // using cc, build and link c code
    cc::Build::new()
        .out_dir("obj")
        .file("c/ozz-animation/src/animation/offline/animation_builder.cc")
        .include("c/ozz-animation/include")
        .include("c/ozz-animation/include/ozz")
        .include("c/ozz-animation/src")
        .cpp(true)
        //.cpp_link_stdlib("stdc++")
        .compile("ozz_cpp");

// csbindgen code, generate both rust ffi and C# dll import
    csbindgen::Builder::default()
        .input_bindgen_file("src/ozz_cpp.rs")            // read from bindgen generated code
        .rust_file_header("use super::ozz_cpp::*;")     // import bindgen generated modules(struct/method)
        .csharp_entry_point_prefix("csbindgen_")
        .csharp_dll_name("primrose_rust")
        .csharp_dll_name_if("PRIMROSE_IOS", "__Internal")
        .csharp_namespace("Primrose.Native.ozz_cpp")
        .csharp_entry_point_prefix("csbindgen_")
        .csharp_class_name("NativeMethods")
        .csharp_method_prefix("ozz_")
        .always_included_types(&["ozz_animation_offline_RawAnimation_TranslationKey", "ozz_animation_offline_RawAnimation_RotationKey", "ozz_animation_offline_RawAnimation_ScaleKey"])
        .generate_to_file("src/ozz_cpp_ffi.rs", "../PrimroseEngine/Engine/Source/Native/Native_ozz_cpp.g.cs")

        .unwrap();*/
}

fn add_bc7e()
{
    ispc::Config::new()
    .target_isas(vec![
        TargetISA::SSE2i32x8,
        TargetISA::SSE4i32x8,
        TargetISA::AVX1i32x16,
        TargetISA::AVX2i32x16,
        TargetISA::AVX512KNLi32x16,
        TargetISA::AVX512SKXi32x16,
    ])
    .out_dir("obj")
    .file("c/bc7e/bc7e.ispc")
    .wno_perf()
    .woff()
    .compile("bc7e");

    csbindgen::Builder::default()
        .input_bindgen_file("src/bc7e.rs")

        .rust_file_header("mod ffi {
    ispc::ispc_module!(bc7e);
}

pub use ffi::bc7e::*;")


        .csharp_dll_name("primrose_rust")
        .csharp_dll_name_if("PRIMROSE_IOS", "__Internal")
        .csharp_namespace("Primrose.Native.bc7e")
        .csharp_entry_point_prefix("csbindgen_")
        .csharp_class_name("NativeMethods")
        //.method_filter(|x| { x.starts_with("bc7e_") } )
        .generate_to_file("src/bc7e_ffi.rs", "../PrimroseEngine/Engine/Source/Native/Native_bc7e.g.cs")
        .unwrap();
}

pub fn add_parley() {
    /*
    csbindgen::Builder::default()
        .input_extern_file("src/parley_ffi.rs")
        .csharp_dll_name("primrose_rust")
        .csharp_dll_name_if("PRIMROSE_IOS", "__Internal")
        .csharp_namespace("Primrose.Native.parley")
        .csharp_class_name("NativeMethods")
        //.method_filter(|x| { x.starts_with("bc7e_") } )
        .generate_csharp_file("../PrimroseEngine/Engine/Source/Native/Native_parley.g.cs")
        .unwrap();*/
}

pub fn add_minimp4() {
/*
    // using bindgen, generate binding code
    bindgen::Builder::default()
        //.clang_arg("-x c++")
        .clang_args(["-x", "c++"])
        .header("c/minimp4/minimp4.h")
        .opaque_type("std::.*")
        .allowlist_function("MP4D_.*")
        .generate().unwrap()
        .write_to_file("src/minimp4.rs").unwrap();

// using cc, build and link c code
    cc::Build::new()
        .out_dir("obj")
        .file("c/minimp4/minimp4.c")
        .cpp(true)
        //.cpp_link_stdlib("stdc++")
        .compile("minimp4");

// csbindgen code, generate both rust ffi and C# dll import
    csbindgen::Builder::default()
        .input_bindgen_file("src/minimp4.rs")            // read from bindgen generated code
        .rust_file_header("use super::minimp4::*;")     // import bindgen generated modules(struct/method)
        .csharp_entry_point_prefix("csbindgen_")
        .csharp_dll_name("primrose_rust")
        .csharp_dll_name_if("PRIMROSE_IOS", "__Internal")
        .csharp_namespace("Primrose.Native.minimp4")
        .csharp_entry_point_prefix("csbindgen_")
        .csharp_class_name("NativeMethods")
        .generate_to_file("src/minimp4_ffi.rs", "../PrimroseEngine/Engine/Source/Native/Native_minimp4.g.cs")
        .unwrap();*/
}

pub fn add_ozz() {
    /*
    csbindgen::Builder::default()
        .input_extern_file("src/ozz_ffi.rs")
        .csharp_dll_name("primrose_rust")
        .csharp_dll_name_if("PRIMROSE_IOS", "__Internal")
        .csharp_namespace("Primrose.Native.ozz")
        .csharp_class_name("NativeMethods")
        //.method_filter(|x| { x.starts_with("bc7e_") } )
        .generate_csharp_file("../PrimroseEngine/Engine/Source/Native/Native_ozz.g.cs")
        .unwrap();*/
}

pub fn add_bytebuffer() {
    csbindgen::Builder::default()
        .input_extern_file("src/bytebuffer.rs")
        .csharp_dll_name("primrose_rust")
        .csharp_dll_name_if("PRIMROSE_IOS", "__Internal")
        .csharp_namespace("Primrose.Native")
        .csharp_class_name("NativeMethods")
        //.method_filter(|x| { x.starts_with("bc7e_") } )
        .generate_csharp_file("../PrimroseEngine/Engine/Source/Native/Native_bytebuffer.g.cs")
        .unwrap();
}

pub fn add_kdtree() {
    csbindgen::Builder::default()
        .input_extern_file("src/kdtree_ffi.rs")
        .csharp_dll_name("primrose_rust")
        .csharp_dll_name_if("PRIMROSE_IOS", "__Internal")
        .csharp_namespace("Primrose.Native.kdtree")
        .csharp_class_name("NativeMethods")
        .generate_csharp_file("../PrimroseEngine/Engine/Source/Native/Native_kdtree.g.cs")
        .unwrap();
}

#[cfg(target_os = "linux")]
fn add_linux() {
    add_ozz();
    add_ozz_cpp();
    add_minimp4();
    add_bytebuffer();
    add_kdtree();
    add_parley();
    add_pitchtracker();
    add_bc7e();
}

#[cfg(target_os = "windows")]
fn add_windows() {
    add_ozz();
    add_ozz_cpp();
    add_minimp4();
    add_bytebuffer();
    add_kdtree();
    add_parley();
    add_pitchtracker();
    add_bc7e();
    add_offsetallocator();
}

#[cfg(target_os = "macos")]
fn add_macos() {
    add_ozz();
    add_ozz_cpp();
    add_minimp4();
    add_bytebuffer();
    add_kdtree();
    add_parley();
    add_pitchtracker();
    add_bc7e();
}

#[cfg(target_os = "ios")]
fn add_ios() {
    add_ozz();
    add_ozz_cpp();
    add_minimp4();
    add_bytebuffer();
    add_kdtree();
    add_parley();
    add_pitchtracker();
}

fn main() {

    #[cfg(target_os = "windows")]
    add_windows();

    #[cfg(target_os = "linux")]
    add_linux();
    
    #[cfg(target_os = "macos")]
    add_macos();

    #[cfg(target_os = "ios")]
    add_ios();
}