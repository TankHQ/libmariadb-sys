use std::env;
use std::path::PathBuf;

fn main() {
    // Configure CMake
    let dst = cmake::Config::new("source")
        .define("WITH_SSL", "OPENSSL")
        .profile("Release")
        .build();
    println!(
        "cargo:rustc-link-search=native={}/lib/mariadb",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=mariadbclient");
    println!("cargo:rerun-if-changed=source/include");

    let bindings = bindgen::Builder::default()
        .header(dst.join("include/mariadb/mysql.h").to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("bindings/bindings.rs")
        .expect("Couldn't write bindings!");
}
