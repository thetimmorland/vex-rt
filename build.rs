use std::env;
use std::path;
use std::process;
use std::str;

use bindgen;
use zip_extensions::zip_extract;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let out_dir = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    extract_firmware(&out_dir);
    generate_bindings(&out_dir);
}

fn extract_firmware(out_dir: &path::PathBuf) {
    let path = path::PathBuf::from("kernel@3.3.1.zip");
    zip_extract(&path, &out_dir).unwrap();

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!(
        "cargo:rustc-link-search={}",
        out_dir.join("firmware").display()
    );
}

fn generate_bindings(out_dir: &path::PathBuf) {
    let err_msg = "failed to execute arm-none-eabi-gcc. is the arm-none-eabi toolchain installed?";

    // https://stackoverflow.com/questions/17939930/finding-out-what-the-gcc-include-path-is
    let output = process::Command::new("arm-none-eabi-gcc")
        .args(&["-E", "-Wp,-v", "-xc", "/dev/null"])
        .output()
        .expect(err_msg);

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

    let stderr = str::from_utf8(&output.stderr).unwrap();

    for line in stderr.lines() {
        if line == "#include <...> search starts here:" {
            in_include_section = true;
        } else if line == "End of search list." {
            in_include_section = false;
        } else if in_include_section {
            include_paths.push(format!("-I{}", line.trim()));
        }
    }

    println!("include_paths: {:#?}", include_paths);

    let bindings = bindgen::Builder::default()
        .header(out_dir.join("include/pros/motors.h").to_str().unwrap())
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

    bindings.write_to_file(out_dir.join("bindings.rs")).unwrap();
}
