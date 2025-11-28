fn main() {
    let mut config = cmake::Config::new("source");
    if cfg!(feature = "with-asan") {
        config.define("WITH_ASAN", "ON");
    }
    if cfg!(feature = "with-ubsan") {
        config.define("WITH_UBSAN", "ON");
    }
    if cfg!(feature = "with-tsan") {
        config.define("WITH_TSAN", "ON");
    }
    if cfg!(feature = "with-msan") {
        config.define("WITH_MSAN", "ON");
    }
    let dst = config
        .define("WITH_SSL", "OPENSSL")
        .profile("Release")
        .build();
    println!(
        "cargo:rustc-link-search=native={}/lib/mariadb",
        dst.display()
    );
    if cfg!(feature = "dynamic") {
        println!("cargo:rustc-link-lib=static=mariadbclient");
    } else {
        println!("cargo:rustc-link-lib=static=mariadbclient");
    }
    println!("cargo:rerun-if-changed=source/include");

    #[cfg(feature = "buildtime-bindgen")]
    {
        use std::env;
        let bindings = bindgen::Builder::default()
            .header(dst.join("include/mariadb/mysql.h").to_str().unwrap())
            .generate()
            .expect("Unable to generate bindings");
        bindings
            .write_to_file(
                std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
                    .join("bindings.rs"),
            )
            .expect("Couldn't write bindings!");
    }
}
