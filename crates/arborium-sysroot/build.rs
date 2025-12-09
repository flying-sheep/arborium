fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = std::path::Path::new(&manifest_dir);

    // wasm-sysroot is bundled inside this crate
    let wasm_sysroot = manifest_path.join("wasm-sysroot");

    // Emit metadata that dependent crates can access via DEP_ARBORIUM_SYSROOT_PATH
    println!("cargo::metadata=PATH={}", wasm_sysroot.display());
}
