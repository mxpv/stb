/// Revised Perlin noise (3D input, 1D output)
use stb_sys as sys;

/// This function computes a random value at the coordinate (x,y,z).
/// Adjacent random values are continuous but the noise fluctuates
/// its randomness with period 1, i.e. takes on wholly unrelated values
/// at integer points. Specifically, this implements Ken Perlin's
/// revised noise function from 2002.
///
/// The "wrap" parameters can be used to create wraparound noise that
/// wraps at powers of two. The numbers MUST be powers of two. Specify
/// 0 to mean "don't care". (The noise always wraps every 256 due
/// details of the implementation, even if you ask for larger or no
/// wrapping.)
pub fn stb_perlin_noise3(x: f32, y: f32, z: f32, x_wrap: i32, y_wrap: i32, z_wrap: i32) -> f32 {
    unsafe { sys::stb_perlin_noise3(x, y, z, x_wrap, y_wrap, z_wrap) }
}

/// Same as `stb_perlin_noise3`, but `seed` selects from multiple different variations of the
/// noise function.
/// The current implementation only uses the bottom 8 bits of 'seed', but possibly in the future
/// more bits will be used.
pub fn stb_perlin_noise3_seed(
    x: f32,
    y: f32,
    z: f32,
    x_wrap: i32,
    y_wrap: i32,
    z_wrap: i32,
    seed: i32,
) -> f32 {
    unsafe { sys::stb_perlin_noise3_seed(x, y, z, x_wrap, y_wrap, z_wrap, seed) }
}

pub fn stb_perlin_ridge_noise3(
    x: f32,
    y: f32,
    z: f32,
    lacunarity: f32,
    gain: f32,
    offset: f32,
    octaves: i32,
) -> f32 {
    unsafe { sys::stb_perlin_ridge_noise3(x, y, z, lacunarity, gain, offset, octaves) }
}

pub fn stb_perlin_fbm_noise3(
    x: f32,
    y: f32,
    z: f32,
    lacunarity: f32,
    gain: f32,
    octaves: i32,
) -> f32 {
    unsafe { sys::stb_perlin_fbm_noise3(x, y, z, lacunarity, gain, octaves) }
}

pub fn stb_perlin_turbulence_noise3(
    x: f32,
    y: f32,
    z: f32,
    lacunarity: f32,
    gain: f32,
    octaves: i32,
) -> f32 {
    unsafe { sys::stb_perlin_turbulence_noise3(x, y, z, lacunarity, gain, octaves) }
}

pub fn stb_perlin_noise3_wrap_nonpow2(
    x: f32,
    y: f32,
    z: f32,
    x_wrap: i32,
    y_wrap: i32,
    z_wrap: i32,
    seed: u8,
) -> f32 {
    unsafe { sys::stb_perlin_noise3_wrap_nonpow2(x, y, z, x_wrap, y_wrap, z_wrap, seed) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perlin_noise3() {
        let n = stb_perlin_noise3(0.5, 0.0, 0.01, 0, 0, 0);
        assert!(n > 0.0);
    }

    #[test]
    fn perlin_noise3_seed() {
        let n = stb_perlin_noise3_seed(0.01, 0.02, 0.03, 0, 0, 0, 123);
        assert!(n > 0.0);
    }

    #[test]
    fn perlin_ridge_noise3() {
        let n = stb_perlin_ridge_noise3(0.02, 0.04, 0.02, 2.0, 0.5, 1.0, 6);
        assert!(n > 0.0);
    }

    #[test]
    fn perlin_fbm_noise3() {
        let n = stb_perlin_fbm_noise3(0.4, 0.1, 0.3, 2.0, 0.5, 6);
        assert!(n > 0.0);
    }

    #[test]
    fn perlin_turbulence_noise3() {
        let n = stb_perlin_turbulence_noise3(0.04, 0.09, 0.91, 2.0, 0.5, 6);
        assert!(n > 0.0);
    }

    #[test]
    fn perlin_noise3_wrap_nonpow2() {
        let n = stb_perlin_noise3_wrap_nonpow2(0.04, 0.09, 0.91, 1, 2, 3, 123);
        assert!(n > 0.0);
    }
}
