# stb-sys

[![Crate](https://img.shields.io/crates/v/stb-sys.svg)](https://crates.io/crates/stb-sys)
![CI](https://github.com/mxpv/stb/workflows/CI/badge.svg)
![Crates.io](https://img.shields.io/crates/l/stb-sys)

Low level bindgen generated bindings to [stb libraries](https://github.com/nothings/stb).

The following APIs are currently available:
- `stb_easy_font`
- `stb_dxt`
- `stb_image`
- `stb_image_write`
- `stb_perlin`
- `stb_rect_pack`
- `stb_image_resize`
- `stb_truetype`

For high level bindigns have a look on [stb](https://crates.io/crates/stb) crate.

## Usage

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
stb-sys = { version = "0.4.0", features = ["stb_image", "stb_image_write"] }
```

## Contributing

Contributions are generally welcome. Make sure your changes make sense for this project (if in doubt, open an issue first),
the code is reasonbly tested, and passes the CI checks.
