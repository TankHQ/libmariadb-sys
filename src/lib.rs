#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings {
    #[cfg(feature = "buildtime-bindgen")]
    include!(concat!(env!("OUT_DIR"), "/bindings/bindings.rs"));

    #[cfg(not(feature = "buildtime-bindgen"))]
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings/bindings.rs"));
}

pub use bindings::*;
