use std::env;
use std::path::PathBuf;

static FILES: &[&str] = &[
    "src/dummy.c", // Keep a dummy file to avoid compile errors when no features specified
    #[cfg(feature = "stb_easy_font")]
    "src/stb_easy_font.c",
];

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut builder = bindgen::builder();
    for f in FILES {
        builder = builder.header(*f)
    }
    builder
        .whitelist_function("stb.*")
        .whitelist_type("stb.*")
        .whitelist_var("stb.*")
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings file");

    cc::Build::new().files(FILES).compile("libstb");
}
