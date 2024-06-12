use ispc::TargetISA;

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
    .out_dir("src")
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
    csbindgen::Builder::default()
        .input_extern_file("src/parley_ffi.rs")
        .csharp_dll_name("primrose_rust")
        .csharp_dll_name_if("PRIMROSE_IOS", "__Internal")
        .csharp_namespace("Primrose.Native.parley")
        .csharp_entry_point_prefix("csbindgen_")
        .csharp_class_name("NativeMethods")
        //.method_filter(|x| { x.starts_with("bc7e_") } )
        .generate_csharp_file("../PrimroseEngine/Engine/Source/Native/Native_parley.g.cs")
        .unwrap();
}

#[cfg(target_os = "linux")]
fn add_linux() {
    add_bc7e();
    add_parley();
}

#[cfg(target_os = "windows")]
fn add_windows() {
    add_bc7e();
    add_parley();
}

#[cfg(target_os = "macos")]
fn add_macos() {
    add_bc7e();
    add_parley();
}

#[cfg(target_os = "ios")]
fn add_ios() {
    add_parley();
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