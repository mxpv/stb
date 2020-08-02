//! Fabian "ryg" Giesen's real-time DXT compressor

use stb_sys as sys;

/// DXT compression mode
pub enum CompressionMode {
    /// Default compression mode
    Normal = 0,
    /// Use dithering. dubious win. never use for normal maps and the like!
    Dither = 1,
    /// High quality mode, does two refinement steps instead of 1. ~30-40% slower.
    Highqual = 2,
}

/// Call `stb_compress_dxt_block()` for every block (you must pad) source should be a 4x4 block of
/// RGBA data in row-major order; Alpha channel is not stored if you specify `alpha=0` (but you
/// must supply some constant alpha in the alpha channel).
/// You can turn on dithering and "high quality" using mode.
pub fn stb_compress_dxt_block(
    dest: &mut [u8],
    src_rgba_four_bytes_per_pixel: &[u8],
    alpha: i32,
    mode: CompressionMode,
) {
    debug_assert_ne!(src_rgba_four_bytes_per_pixel.len(), 0);
    debug_assert_eq!(src_rgba_four_bytes_per_pixel.len() % 16, 0);
    debug_assert_ne!(dest.len(), 0);
    debug_assert_eq!(dest.len() % 8, 0);
    unsafe {
        sys::stb_compress_dxt_block(
            dest.as_mut_ptr(),
            src_rgba_four_bytes_per_pixel.as_ptr(),
            alpha,
            mode as i32,
        )
    }
}

pub fn stb_compress_bc4_block(dest: &mut [u8], src_r_one_byte_per_pixel: &[u8]) {
    unsafe { sys::stb_compress_bc4_block(dest.as_mut_ptr(), src_r_one_byte_per_pixel.as_ptr()) }
}

pub fn stb_compress_bc5_block(dest: &mut [u8], src_rg_two_byte_per_pixel: &[u8]) {
    unsafe { sys::stb_compress_bc5_block(dest.as_mut_ptr(), src_rg_two_byte_per_pixel.as_ptr()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_dxt_block() {
        let src: [u8; 16] = [
            255, 255, 0, 128, 255, 0, 128, 255, 255, 128, 0, 128, 128, 0, 128, 255,
        ];
        let mut dst: [u8; 8] = [0; 8];
        stb_compress_dxt_block(&mut dst, &src, 0, CompressionMode::Normal);
    }
}
