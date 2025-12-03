use std::env;
use std::path::PathBuf;

static FILES: &[&str] = &[
    #[cfg(feature = "stb_easy_font")]
    "src/stb_easy_font.c",
    #[cfg(feature = "stb_dxt")]
    "src/stb_dxt.c",
    #[cfg(feature = "stb_image")]
    "src/stb_image.c",
    #[cfg(feature = "stb_image_write")]
    "src/stb_image_write.c",
    #[cfg(feature = "stb_rect_pack")]
    "src/stb_rect_pack.c",
    #[cfg(feature = "stb_image_resize")]
    "src/stb_image_resize.c",
    #[cfg(feature = "stb_truetype")]
    "src/stb_truetype.c",
];

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_dir.join("bindings.rs");

    if FILES.is_empty() {
        // Write an empty file so `include!` won't fail the build
        std::fs::write(bindings_path, "").unwrap();
        return;
    }

    let mut builder = bindgen::builder();
    for f in FILES {
        builder = builder.header(*f)
    }
    builder
        .allowlist_function("stb.*")
        .allowlist_type("stb.*")
        .allowlist_var("stb.*")
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(bindings_path)
        .expect("Failed to write bindings file");

    let mut builder = cc::Build::new();

    #[cfg(feature = "stb_dxt")]
    {
        #[cfg(feature = "stb_dxt_use_rounding_bias")]
        builder.define("STB_DXT_USE_ROUNDING_BIAS", "1");
    }

    #[cfg(feature = "stb_image")]
    {
        #[cfg(feature = "stbi_no_linear")]
        builder.define("STBI_NO_LINEAR", "1");

        #[cfg(feature = "stbi_no_jpeg")]
        builder.define("STBI_NO_JPEG", "1");

        #[cfg(feature = "stbi_no_png")]
        builder.define("STBI_NO_PNG", "1");

        #[cfg(feature = "stbi_no_bmp")]
        builder.define("STBI_NO_BMP", "1");

        #[cfg(feature = "stbi_no_psd")]
        builder.define("STBI_NO_PSD", "1");

        #[cfg(feature = "stbi_no_gif")]
        builder.define("STBI_NO_GIF", "1");

        #[cfg(feature = "stbi_no_hdr")]
        builder.define("STBI_NO_HDR", "1");

        #[cfg(feature = "stbi_no_pic")]
        builder.define("STBI_NO_PIC", "1");

        #[cfg(feature = "stbi_no_pnm")]
        builder.define("STBI_NO_PNM", "1");
    }

    builder.files(FILES).warnings(false).compile("libstb");
}
