# stb

All in one safe Rust API and wrappers for [stb libraries](https://github.com/nothings/stb).

The following APIs are currently available:
- `stb_easy_font`
- `stb_dxt`
- `stb_image`

## Usage

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
stb = "0.1.0"
```

or if you want to have a specific feature selection:

```toml
[dependencies]
stb = { version = "0.1.0", default-features = false, features = ["stb_easy_font", "stb_dxt", "stb_image"] }
```

## Create features
Here is the list of feature toggles available in this crate so far:
- `stb_easy_font`
- `stb_dxt`
    * `stb_dxt_use_rounding_bias`
- `stb_image`
    * `stbi_no_linear`
    * `stbi_no_jpeg`
    * `stbi_no_png`
    * `stbi_no_bmp`
    * `stbi_no_psd`
    * `stbi_no_gif`
    * `stbi_no_hdr`
    * `stbi_no_pic`
    * `stbi_no_pnm`

## Implementation notes

- `stb_easy_font`
    * `stb_easy_font_print` accepts a buffer for quads with the size of your choice. Currently `stb` offers no API to
    predict buffer's size depending on text string. If the buffer is not large enought, quads will be truncated.
- `stb_image`
    * The crate wraps `stbi_io_callbacks` with a generic reader (anything that implements `io::Read` and `io::Seek`).
    So look for `stbi_xyz_from_reader` APIs instead of `stbi_xyz_from_callbacks`.
    * There is no `Stdio` version of the API since it is convenient enough to use `stbi_xyz_from_reader` API from Rust
    and there is no need to pay C string conversion overhead.
    * You can use `stbi_no_FORMAT` feature toggles to disable not needed image formats.
