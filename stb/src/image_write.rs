//! A module for writing images to C stdio or a callback.
//!
//! The PNG output is not optimal; it is 20-50% larger than the file
//! written by a decent optimizing implementation; though providing a custom
//! zlib compress function (see STBIW_ZLIB_COMPRESS) can mitigate that.
//! This library is designed for source code compactness and simplicity,
//! not optimal image file size or run-time performance.

use stb_sys as sys;
use std::ffi::c_void;
use std::ffi::CStr;
use std::os::raw;
use std::slice;

pub fn stbi_write_png(
    filename: &CStr,
    w: i32,
    h: i32,
    comp: i32,
    buffer: &[u8],
    stride_in_bytes: i32,
) -> Option<()> {
    let ret = unsafe {
        sys::stbi_write_png(
            filename.as_ptr() as *mut i8,
            w,
            h,
            comp,
            buffer.as_ptr() as *const c_void,
            stride_in_bytes,
        )
    };
    // Each function returns 0 on failure and non-0 on success.
    if ret == 0 {
        None
    } else {
        Some(())
    }
}

pub fn stbi_write_bmp(filename: &CStr, w: i32, h: i32, comp: i32, buffer: &[u8]) -> Option<()> {
    let ret = unsafe {
        sys::stbi_write_bmp(
            filename.as_ptr() as *mut i8,
            w,
            h,
            comp,
            buffer.as_ptr() as *const c_void,
        )
    };
    // Each function returns 0 on failure and non-0 on success.
    if ret == 0 {
        None
    } else {
        Some(())
    }
}

pub fn stbi_write_tga(filename: &CStr, w: i32, h: i32, comp: i32, buffer: &[u8]) -> Option<()> {
    let ret = unsafe {
        sys::stbi_write_tga(
            filename.as_ptr() as *mut i8,
            w,
            h,
            comp,
            buffer.as_ptr() as *const c_void,
        )
    };
    // Each function returns 0 on failure and non-0 on success.
    if ret == 0 {
        None
    } else {
        Some(())
    }
}

pub fn stbi_write_hdr(filename: &CStr, w: i32, h: i32, comp: i32, buffer: &[f32]) -> Option<()> {
    let ret =
        unsafe { sys::stbi_write_hdr(filename.as_ptr() as *mut i8, w, h, comp, buffer.as_ptr()) };
    // Each function returns 0 on failure and non-0 on success.
    if ret == 0 {
        None
    } else {
        Some(())
    }
}

pub fn stbi_write_jpg(
    filename: &CStr,
    w: i32,
    h: i32,
    comp: i32,
    buffer: &[u8],
    quality: i32,
) -> Option<()> {
    let ret = unsafe {
        sys::stbi_write_jpg(
            filename.as_ptr() as *mut i8,
            w,
            h,
            comp,
            buffer.as_ptr() as *const c_void,
            quality,
        )
    };
    // Each function returns 0 on failure and non-0 on success.
    if ret == 0 {
        None
    } else {
        Some(())
    }
}

extern "C" fn write_func<F, T>(context: *mut raw::c_void, data: *mut raw::c_void, size: raw::c_int)
where
    F: FnMut(&[T]),
{
    let buffer = unsafe { slice::from_raw_parts_mut(data as *mut T, size as _) };
    // See https://s3.amazonaws.com/temp.michaelfbryan.com/callbacks/index.html
    let f: &mut F = unsafe { &mut *(context as *mut F) };

    f(buffer)
}

pub fn stbi_write_png_to_func<F>(
    func: &mut F,
    w: i32,
    h: i32,
    comp: i32,
    buffer: &[u8],
    stride_in_bytes: i32,
) -> Option<()>
where
    F: FnMut(&[u8]),
{
    let ret = unsafe {
        sys::stbi_write_png_to_func(
            Some(write_func::<F, u8>),
            func as *mut F as *mut c_void,
            w,
            h,
            comp,
            buffer.as_ptr() as *const c_void,
            stride_in_bytes,
        )
    };

    if ret == 0 {
        None
    } else {
        Some(())
    }
}

pub fn stbi_write_bmp_to_func<F>(
    func: &mut F,
    w: i32,
    h: i32,
    comp: i32,
    buffer: &[u8],
) -> Option<()>
where
    F: FnMut(&[u8]),
{
    let ret = unsafe {
        sys::stbi_write_bmp_to_func(
            Some(write_func::<F, u8>),
            func as *mut F as *mut c_void,
            w,
            h,
            comp,
            buffer.as_ptr() as *const c_void,
        )
    };

    if ret == 0 {
        None
    } else {
        Some(())
    }
}

pub fn stbi_write_tga_to_func<F>(
    func: &mut F,
    w: i32,
    h: i32,
    comp: i32,
    buffer: &[u8],
) -> Option<()>
where
    F: FnMut(&[u8]),
{
    let ret = unsafe {
        sys::stbi_write_tga_to_func(
            Some(write_func::<F, u8>),
            func as *mut F as *mut c_void,
            w,
            h,
            comp,
            buffer.as_ptr() as *const c_void,
        )
    };

    if ret == 0 {
        None
    } else {
        Some(())
    }
}

pub fn stbi_write_hdr_to_func<F>(
    func: &mut F,
    w: i32,
    h: i32,
    comp: i32,
    buffer: &[f32],
) -> Option<()>
where
    F: FnMut(&[f32]),
{
    let ret = unsafe {
        sys::stbi_write_hdr_to_func(
            Some(write_func::<F, f32>),
            func as *mut F as *mut c_void,
            w,
            h,
            comp,
            buffer.as_ptr() as *const f32,
        )
    };

    if ret == 0 {
        None
    } else {
        Some(())
    }
}

pub fn stbi_write_jpg_to_func<F>(
    func: &mut F,
    w: i32,
    h: i32,
    comp: i32,
    buffer: &[u8],
    quality: i32,
) -> Option<()>
where
    F: FnMut(&[u8]),
{
    let ret = unsafe {
        sys::stbi_write_jpg_to_func(
            Some(write_func::<F, u8>),
            func as *mut F as *mut c_void,
            w,
            h,
            comp,
            buffer.as_ptr() as *const c_void,
            quality,
        )
    };

    if ret == 0 {
        None
    } else {
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::fs;

    #[test]
    fn write_bmp() {
        let mut dir = std::env::temp_dir();
        dir.push("test.bmp");
        let str = dir.to_str().unwrap();
        let path = CString::new(str).unwrap();

        stbi_write_bmp(&path, 1, 1, 1, &[1]).expect("Failed to write BMP");

        // Make sure file exists
        fs::metadata(str).expect("Failed to check whether BMP file exists");
        fs::remove_file(str).expect("Failed to remove BMP file");
    }

    #[test]
    fn write_bmp_callback() {
        let mut counter = 0;
        stbi_write_bmp_to_func(
            &mut |_data| {
                counter += 1;
            },
            1,
            1,
            1,
            &[1],
        )
        .expect("Failed to write BMP to func");
        assert_ne!(counter, 0);
    }
}
