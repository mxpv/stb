use stb_sys as sys;
use std::cmp::Ordering;
use std::ffi;
use std::io;
use std::os::raw;
use std::slice;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Channels {
    Default = 0,
    Grey = 1,
    GreyAlpha = 2,
    Rgb = 3,
    RgbAlpha = 4,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Info {
    /// Image width in pixels
    pub width: i32,
    /// Image height in pixels
    pub height: i32,
    /// Number of image components in image file
    pub components: i32,
}

pub struct Data<T> {
    data: *mut T,
    size: usize,
}

impl<T> Data<T> {
    fn new(data: *mut T, desired_channels: Channels, info: Info) -> Self {
        let components = if desired_channels == Channels::Default {
            info.components
        } else {
            desired_channels as i32
        };

        let size = (info.width * info.height * components) as usize;

        Data { data, size }
    }

    pub fn as_slice(&self) -> &[T] {
        let size = self.size();
        unsafe { slice::from_raw_parts(self.data, size) }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<T> Drop for Data<T> {
    fn drop(&mut self) {
        unsafe { sys::stbi_image_free(self.data as *mut ffi::c_void) };
    }
}

/// IO wrapper for stb
struct Wrapper<'a, R> {
    reader: &'a mut R,
    err: bool,
}

impl<'a, R> Wrapper<'a, R>
where
    R: io::Read + io::Seek,
{
    fn new(reader: &'a mut R) -> (Wrapper<'a, R>, sys::stbi_io_callbacks) {
        let reader = Wrapper { reader, err: false };
        let callbacks = sys::stbi_io_callbacks {
            read: Some(Self::io_read),
            skip: Some(Self::io_skip),
            eof: Some(Self::io_eof),
        };

        (reader, callbacks)
    }

    fn from_user_data(user: *mut raw::c_void) -> &'a mut Wrapper<'a, R> {
        unsafe { &mut *(user as *mut Wrapper<R>) }
    }

    fn read(&mut self, data: *mut raw::c_char, size: raw::c_int) -> raw::c_int {
        if self.err {
            return 0;
        }

        let dest = unsafe {
            let data = data as *mut u8;
            slice::from_raw_parts_mut(data, size as _)
        };

        if let Ok(n) = self.reader.read(dest) {
            return n as _;
        }

        self.err = true;
        0
    }

    fn skip(&mut self, n: raw::c_int) {
        match n.cmp(&0) {
            Ordering::Greater => {
                if self.reader.seek(io::SeekFrom::Current(n as _)).is_err() {
                    self.err = true
                }
            }
            Ordering::Less => {
                // stb allows negative seeks while Rust API considers this as an error
                if self
                    .reader
                    .seek(io::SeekFrom::Current(0)) // Find current position
                    .and_then(|pos| self.reader.seek(io::SeekFrom::Start(pos - n as u64))) // Seek from start
                    .is_err()
                {
                    self.err = true
                }
            }
            _ => {
                // Do nothing if zero
            }
        }
    }

    fn eof(&self) -> raw::c_int {
        if self.err {
            1
        } else {
            0
        }
    }

    extern "C" fn io_read(
        user: *mut raw::c_void,
        data: *mut raw::c_char,
        size: raw::c_int,
    ) -> raw::c_int {
        Wrapper::<R>::from_user_data(user).read(data, size)
    }

    extern "C" fn io_skip(user: *mut raw::c_void, n: raw::c_int) {
        Wrapper::<R>::from_user_data(user).skip(n);
    }

    extern "C" fn io_eof(user: *mut raw::c_void) -> raw::c_int {
        Wrapper::<R>::from_user_data(user).eof()
    }
}

/// By default we convert iphone-formatted PNGs back to RGB, even though they are internally
/// encoded differently. You can disable this conversion by calling
/// `stbi_convert_iphone_png_to_rgb(false)`, in which case you will always just get the
/// native iphone "format" through (which is BGR stored in RGB).
pub fn stbi_convert_iphone_png_to_rgb(true_if_should_convert: bool) {
    unsafe { sys::stbi_convert_iphone_png_to_rgb(if true_if_should_convert { 1 } else { 0 }) }
}

/// Call `stbi_set_unpremultiply_on_load(true)` to force a divide per pixel to remove any
/// premultiplied alpha *only* if the image file explicitly says there's premultiplied
/// data (currently only happens in iPhone images, and only if iPhone convert-to-rgb processing is on).
pub fn stbi_set_unpremultiply_on_load(true_if_should_unpremultiply: bool) {
    unsafe { sys::stbi_set_unpremultiply_on_load(if true_if_should_unpremultiply { 1 } else { 0 }) }
}

/// Flip the image vertically, so the first pixel in the output array is the bottom left
pub fn stbi_set_flip_vertically_on_load(true_if_should_flip: bool) {
    unsafe { sys::stbi_set_flip_vertically_on_load(if true_if_should_flip { 1 } else { 0 }) }
}

/// Get image dimensions & components from a slice without fully decoding
pub fn stbi_info_from_memory(buffer: &[u8]) -> Option<Info> {
    let mut info = Info::default();
    let ret = unsafe {
        sys::stbi_info_from_memory(
            buffer.as_ptr(),
            buffer.len() as i32,
            &mut info.width,
            &mut info.height,
            &mut info.components,
        )
    };
    if ret == 0 {
        None
    } else {
        Some(info)
    }
}

/// Get image dimensions & components from reader without fully decoding
pub fn stbi_info_from_reader<R>(reader: &mut R) -> Option<Info>
where
    R: io::Read + io::Seek,
{
    let (mut reader, callbacks) = Wrapper::new(reader);
    let mut info = Info::default();

    let ret = unsafe {
        sys::stbi_info_from_callbacks(
            &callbacks,
            &mut reader as *mut _ as *mut ffi::c_void,
            &mut info.width,
            &mut info.height,
            &mut info.components,
        )
    };

    if ret == 0 {
        None
    } else {
        Some(info)
    }
}

pub fn stbi_is_16_bit_from_memory(buffer: &[u8]) -> bool {
    let ret = unsafe { sys::stbi_is_16_bit_from_memory(buffer.as_ptr(), buffer.len() as i32) };
    ret == 1
}

pub fn stbi_is_16_bit_from_reader<R>(reader: &mut R) -> bool
where
    R: io::Read + io::Seek,
{
    let (mut reader, callbacks) = Wrapper::new(reader);
    let ret = unsafe {
        sys::stbi_is_16_bit_from_callbacks(&callbacks, &mut reader as *mut _ as *mut ffi::c_void)
    };
    ret == 1
}

pub fn stbi_load_from_memory(
    buffer: &[u8],
    desired_channels: Channels,
) -> Option<(Info, Data<u8>)> {
    let mut info = Info::default();

    let data = unsafe {
        sys::stbi_load_from_memory(
            buffer.as_ptr(),
            buffer.len() as i32,
            &mut info.width,
            &mut info.height,
            &mut info.components,
            desired_channels as i32,
        )
    };

    if data.is_null() {
        None
    } else {
        Some((info, Data::new(data, desired_channels, info)))
    }
}

pub fn stbi_load_from_reader<R>(
    reader: &mut R,
    desired_channels: Channels,
) -> Option<(Info, Data<u8>)>
where
    R: io::Read + io::Seek,
{
    let (mut reader, callbacks) = Wrapper::new(reader);
    let mut info = Info::default();

    let data = unsafe {
        sys::stbi_load_from_callbacks(
            &callbacks,
            &mut reader as *mut _ as *mut ffi::c_void,
            &mut info.width,
            &mut info.height,
            &mut info.components,
            desired_channels as i32,
        )
    };

    if data.is_null() {
        None
    } else {
        Some((info, Data::new(data, desired_channels, info)))
    }
}

pub fn stbi_load_16_from_memory(
    buffer: &[u8],
    desired_channels: Channels,
) -> Option<(Info, Data<u16>)> {
    let mut info = Info::default();

    let data = unsafe {
        sys::stbi_load_16_from_memory(
            buffer.as_ptr(),
            buffer.len() as i32,
            &mut info.width,
            &mut info.height,
            &mut info.components,
            desired_channels as i32,
        )
    };

    if data.is_null() {
        None
    } else {
        Some((info, Data::new(data, desired_channels, info)))
    }
}

pub fn stbi_load_16_from_reader<R>(
    reader: &mut R,
    desired_channels: Channels,
) -> Option<(Info, Data<u16>)>
where
    R: io::Read + io::Seek,
{
    let (mut reader, callbacks) = Wrapper::new(reader);
    let mut info = Info::default();

    let data = unsafe {
        sys::stbi_load_16_from_callbacks(
            &callbacks,
            &mut reader as *mut _ as *mut ffi::c_void,
            &mut info.width,
            &mut info.height,
            &mut info.components,
            desired_channels as i32,
        )
    };

    if data.is_null() {
        None
    } else {
        Some((info, Data::new(data, desired_channels, info)))
    }
}

#[cfg(not(feature = "stbi_no_linear"))]
pub fn stbi_loadf_from_memory(
    buffer: &[u8],
    desired_channels: Channels,
) -> Option<(Info, Data<f32>)> {
    let mut info = Info::default();

    let data = unsafe {
        sys::stbi_loadf_from_memory(
            buffer.as_ptr(),
            buffer.len() as i32,
            &mut info.width,
            &mut info.height,
            &mut info.components,
            desired_channels as i32,
        )
    };

    if data.is_null() {
        None
    } else {
        Some((info, Data::new(data, desired_channels, info)))
    }
}

#[cfg(not(feature = "stbi_no_linear"))]
pub fn stbi_loadf_from_reader<R>(
    reader: &mut R,
    desired_channels: Channels,
) -> Option<(Info, Data<f32>)>
where
    R: io::Read + io::Seek,
{
    let (mut reader, callbacks) = Wrapper::new(reader);
    let mut info = Info::default();

    let data = unsafe {
        sys::stbi_loadf_from_callbacks(
            &callbacks,
            &mut reader as *mut _ as *mut ffi::c_void,
            &mut info.width,
            &mut info.height,
            &mut info.components,
            desired_channels as i32,
        )
    };

    if data.is_null() {
        None
    } else {
        Some((info, Data::new(data, desired_channels, info)))
    }
}

#[cfg(not(feature = "stbi_no_hdr"))]
pub fn stbi_hdr_to_ldr_gamma(gamma: f32) {
    unsafe { sys::stbi_hdr_to_ldr_gamma(gamma) }
}

#[cfg(not(feature = "stbi_no_hdr"))]
pub fn stbi_hdr_to_ldr_scale(scale: f32) {
    unsafe { sys::stbi_hdr_to_ldr_scale(scale) }
}

#[cfg(not(feature = "stbi_no_linear"))]
pub fn stbi_ldr_to_hdr_gamma(gamma: f32) {
    unsafe { sys::stbi_ldr_to_hdr_gamma(gamma) }
}

#[cfg(not(feature = "stbi_no_linear"))]
pub fn stbi_ldr_to_hdr_scale(scale: f32) {
    unsafe { sys::stbi_ldr_to_hdr_scale(scale) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn fixture_path(file: &str) -> PathBuf {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut path = PathBuf::from(root.parent().unwrap());

        path.push("tests/fixtures");
        path.push(file);

        path
    }

    #[test]
    fn info_from_memory() {
        let data = fs::read(fixture_path("white.png")).expect("Failed to read test file");

        let info = stbi_info_from_memory(&data).expect("Failed to get image info from memory");
        assert_eq!(info.width, 20);
        assert_eq!(info.height, 30);
        assert_eq!(info.components, 1);
    }

    #[test]
    fn info_from_reader() {
        let mut f = fs::File::open(fixture_path("white.png")).expect("Failed to open file reader");
        let info = stbi_info_from_reader(&mut f).expect("Failed to get image info from callbacks");

        assert_eq!(info.width, 20);
        assert_eq!(info.height, 30);
        assert_eq!(info.components, 1);
    }

    #[test]
    fn load_8bit_from_memory() {
        let data = fs::read(fixture_path("white.png")).expect("Failed to read test file");
        let (info, image) =
            stbi_load_from_memory(&data, Channels::Grey).expect("Failed to load image");

        assert_eq!(info.width, 20);
        assert_eq!(info.height, 30);
        assert_eq!(info.components, 1);

        let data = image.as_slice();
        assert_eq!(data.len(), 600);

        for c in data.iter().cloned() {
            assert_eq!(c, 255);
        }
    }

    #[test]
    fn load_8bit_from_reader() {
        let mut f = fs::File::open(fixture_path("white.png")).expect("Failed to open file reader");
        let (info, image) = stbi_load_from_reader(&mut f, Channels::Grey)
            .expect("Failed to load image from reader");

        assert_eq!(info.width, 20);
        assert_eq!(info.height, 30);
        assert_eq!(info.components, 1);

        let data = image.as_slice();
        assert_eq!(data.len(), 600);

        for c in data.iter().cloned() {
            assert_eq!(c, u8::MAX);
        }
    }

    #[test]
    fn load_16bit_from_memory() {
        let data = fs::read(fixture_path("white.png")).expect("Failed to read test file");
        let (info, image) =
            stbi_load_16_from_memory(&data, Channels::Default).expect("Failed to load image");

        assert_eq!(info.width, 20);
        assert_eq!(info.height, 30);
        assert_eq!(info.components, 1);

        let data = image.as_slice();
        assert_eq!(data.len(), 600);

        for c in data.iter().cloned() {
            assert_eq!(c, u16::MAX);
        }
    }

    #[test]
    fn load_16bit_from_memory_remap_channels() {
        let data = fs::read(fixture_path("white.png")).expect("Failed to read test file");
        let (info, image) =
            stbi_load_16_from_memory(&data, Channels::GreyAlpha).expect("Failed to load image");

        assert_eq!(info.width, 20);
        assert_eq!(info.height, 30);
        assert_eq!(info.components, 1);

        let data = image.as_slice();
        assert_eq!(data.len(), 1200);

        for c in data.iter().cloned() {
            assert_eq!(c, u16::MAX);
        }
    }
}
