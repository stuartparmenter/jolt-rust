use std::{env, path::PathBuf};
fn main() {
    if std::env::var_os("CARGO_FEATURE_DOUBLE_PRECISION").is_some()
        || std::env::var_os("CARGO_FEATURE_OBJECT_LAYER_U32").is_some()
    {
        panic!("This compatibility prototype currently supports only single precision / u16 object layers");
    }
    if std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() != Ok("x86_64")
        || std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("linux")
    {
        panic!("This cxx prototype is currently validated only on x86_64 Linux");
    }

    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("../..")
        .canonicalize()
        .unwrap();
    let jolt = root.join("vendor/JoltPhysics");
    assert!(
        jolt.join("Jolt/Jolt.h").is_file(),
        "Run cargo xtask setup first"
    );
    for p in [
        root.join("native"),
        root.join("upstream.toml"),
        root.join("crates/rolt/src/raw/bridge.rs"),
        root.join("crates/rolt/src/jolt.rs"),
    ] {
        println!("cargo:rerun-if-changed={}", p.display());
    }
    let asserts = env::var_os("CARGO_FEATURE_ASSERTS").is_some();
    let dst = cmake::Config::new(root.join("native"))
        .define("JOLT_ROOT", &jolt)
        .define("USE_ASSERTS", if asserts { "ON" } else { "OFF" })
        .profile("Release")
        .build();
    let mut bridge = cxx_build::bridges(["src/raw/bridge.rs", "src/jolt.rs"]);
    if asserts {
        bridge.define("JPH_ENABLE_ASSERTS", None);
    }
    bridge
        .file(root.join("native/shims.cpp"))
        .file(root.join("native/callbacks.cpp"))
        .include(root.join("native"))
        .include(jolt)
        .std("c++20")
        .flag_if_supported("-fno-rtti")
        .define("NDEBUG", None)
        .define("JPH_DEBUG_RENDERER", None)
        .compile("rolt_cxx_bridge");
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=Jolt");
}
