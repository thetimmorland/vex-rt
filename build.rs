use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let _target = env::var("TARGET").unwrap();
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=firmare/libpros.a");
    println!("cargo:rerun-if-changed=firmare/libc.a");
    println!("cargo:rerun-if-changed=firmare/libm.a");
    println!("cargo:rerun-if-changed=firmare/v5-common.ld");
    println!("cargo:rerun-if-changed=firmare/v5.ld.a");

    fs::copy("firmware/libpros.a", out.join("libpros.a")).unwrap();
    fs::copy("firmware/libc.a", out.join("libc.a")).unwrap();
    fs::copy("firmware/libm.a", out.join("libm.a")).unwrap();
    fs::copy("firmware/v5-common.ld", out.join("v5-common.ld")).unwrap();
    fs::copy("firmware/v5.ld", out.join("v5.ld")).unwrap();

    println!("cargo:rustc-link-search={}", out.display());
}
