//! Rust API for stb libraries (see details https://github.com/nothings/stb)
//!
//! Philosophy
//!
//! stb libraries are designed with the following priorities:
//! - Easy to use
//! - Easy to maintain
//! - Good performance
//!
//! Sometimes I let "good performance" creep up in priority over "easy to maintain",
//! and for best performance I may provide less-easy-to-use APIs that give higher
//! performance, in addition to the easy-to-use ones. Nevertheless, it's important
//! to keep in mind that from the standpoint of you, a client of this library,
//! all you care about is #1 and #3, and stb libraries DO NOT emphasize #3 above all.
//!
//! Some secondary priorities arise directly from the first two, some of which
//! provide more explicit reasons why performance can't be emphasized.
//!
//! - Portable ("ease of use")
//! - Small source code footprint ("easy to maintain")
//! - No dependencies ("ease of use")

/// Quick-and-dirty easy-to-deploy bitmap font for printing frame rate, etc
#[cfg(feature = "stb_easy_font")]
pub mod easy_font;

/// Fabian "ryg" Giesen's real-time DXT compressor
#[cfg(feature = "stb_dxt")]
pub mod dxt;

/// Image loading/decoding
#[cfg(feature = "stb_image")]
pub mod image;

/// Image writing to disk: PNG, TGA, BMP
#[cfg(feature = "stb_image_write")]
pub mod image_write;
