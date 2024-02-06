use std::{path::Path, process::Command};

fn main() {
    let current_dir = std::env::current_dir().expect("Couldn't get current directory");

    let dir_offset = "../yew_part"; // update to your directory
    let yew_dir = current_dir.join(dir_offset).canonicalize().expect("Couldn't get path of web component");
    assert!(yew_dir.is_dir());
    // println!("cargo:rerun-if-changed={}", yew_dir.display());
    // don't write into the targets directory for now - figure that out later
    // let out_dir = env::var_os("OUT_DIR").unwrap();

    let output = Command::new("trunk")
        .current_dir(yew_dir.clone())
        .args(&["build", "--release"])
        .arg("index.html")
        .output()
        .expect("Unable to build wasm files successfully");

    if !output.status.success() {
        panic!(
            "Error while compiling:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let dest_path = yew_dir.join("dist");

    let js_file = dest_path.join("yew_part.js");
    let wasm_file = dest_path.join("yew_part_bg.wasm");
    let html_file = dest_path.join("index.html");

    for file in &[&js_file, &wasm_file, &html_file] {
        let file = std::fs::metadata(file).expect("file to exist");
        assert!(file.is_file());
    }

    println!("cargo:rustc-env=YEW_PART_JS={}", js_file.display());
    println!("cargo:rustc-env=YEW_PART_WASM={}", wasm_file.display());
    println!("cargo:rustc-env=YEW_PART_HTML={}", html_file.display());

    // Pass some extra options to rustc, some of which get passed on to the linker.
    // // * linker argument --nmagic turns off page alignment of sections (which saves
    //   flash space)
    // * linker argument -Tlink.x tells the linker to use link.x as the linker
    //   script. This is usually provided by the cortex-m-rt crate, and by default
    //   the version in that crate will include a file called `memory.x` which
    //   describes the particular memory layout for your specific chip.
    // * inline-threshold=5 makes the compiler more aggressive and inlining functions
    // * no-vectorize-loops turns off the loop vectorizer (seeing as the M0+ doesn't
    //   have SIMD)
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Tlink-rp.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");        
}
