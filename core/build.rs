use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str;

use bindgen;

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    //
    // register rerun files
    //

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=kernel/firmare/libpros.a");
    println!("cargo:rerun-if-changed=kernel/firmare/libc.a");
    println!("cargo:rerun-if-changed=kernel/firmare/libm.a");
    println!("cargo:rerun-if-changed=kernel/firmare/v5-common.ld");
    println!("cargo:rerun-if-changed=kernel/firmare/v5.ld.a");

    //
    // find arm-none-eabi include paths
    //

    let err_msg = "Failed to execute arm-none-eabi-gcc. Is the arm-none-eabi toolchain installed?";

    // https://stackoverflow.com/questions/17939930/finding-out-what-the-gcc-include-path-is
    let output = if cfg!(target_os = "windows") {
        Command::new("arm-none-eabi-gcc")
            .args(&["-E", "-Wp,-v", "-xc", "Nul"])
            .output()
            .expect(err_msg)
    } else {
        Command::new("arm-none-eabi-gcc")
            .args(&["-E", "-Wp,-v", "-xc", "/dev/null"])
            .output()
            .expect(err_msg)
    };

    #[rustfmt::skip]
    // what we want is in stderr for some god-forsaken reason
    //
    // On my system it looks like this:
    //
    // #include <...> search starts here:
    // /usr/local/Cellar/arm-none-eabi-gcc/10.1.0/lib/gcc/arm-none-eabi/gcc/arm-none-eabi/10.1.0/include
    // /usr/local/Cellar/arm-none-eabi-gcc/10.1.0/lib/gcc/arm-none-eabi/gcc/arm-none-eabi/10.1.0/include-fixed
    // /usr/local/Cellar/arm-none-eabi-gcc/10.1.0/lib/gcc/arm-none-eabi/gcc/arm-none-eabi/10.1.0/../../../../../../arm-none-eabi/include
    // End of search list.

    let mut in_include_section = false;
    let mut include_paths: Vec<String> = Vec::new();

    let stderr = str::from_utf8(&output.stderr).expect("Could not read output of arm-none-eabi-gcc as utf-8. Create a Github issue if you see this.");

    for line in stderr.lines() {
        if line == "#include <...> search starts here:" {
            in_include_section = true;
        } else if line == "End of search list." {
            in_include_section = false;
        } else if in_include_section {
            include_paths.push(format!("-I{}", line.trim()));
        }
    }

    //
    // generate bindings
    //

    let bindings = bindgen::Builder::default()
        .header("kernel/include/pros/motors.h")
        .whitelist_function("motor_.*")
        .whitelist_type("motor_.*")
        .rustified_enum("motor_.*")
        .clang_arg("-target")
        .clang_arg("arm-none-eabi")
        .clang_args(&include_paths)
        .use_core()
        .ctypes_prefix("libc")
        .layout_tests(false)
        .generate()
        .expect("Could not generate bindings.");

    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    //
    // copy firmware
    //

    fs::create_dir(out.join("firmware")).unwrap_or_default();
    fs::copy("kernel/firmware/libpros.a", out.join("firmware/libpros.a")).unwrap();
    fs::copy("kernel/firmware/libc.a", out.join("firmware/libc.a")).unwrap();
    fs::copy("kernel/firmware/libm.a", out.join("firmware/libm.a")).unwrap();
    fs::copy(
        "kernel/firmware/v5-common.ld",
        out.join("firmware/v5-common.ld"),
    )
    .unwrap();
    fs::copy("kernel/firmware/v5.ld", out.join("firmware/v5.ld")).unwrap();

    // this is for linker to find linker scripts
    println!("cargo:rustc-link-search={}", out.display());

    // this is for linker to find libraries
    println!("cargo:rustc-link-search={}", out.join("firmware").display());
}
