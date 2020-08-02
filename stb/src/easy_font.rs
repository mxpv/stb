//! Intended for when you just want to get some text displaying in a 3D app as quickly as possible.
//!
//! Rust implementation notes:
//!
//! `stb_easy_font_print` accepts a buffer for quads with the size of your choice.
//! Currently `stb` C API offers no way to predict buffer's size depending on text string.
//! If the buffer is not large enought, quads will be truncated.

use stb_sys as sys;
use std::ffi::CStr;
use std::mem::size_of;

/// Font quad vertex.
/// You can ignore z and color if you get them from elsewhere.
/// This format was chosen in the hopes it would make it easier for you to reuse existing
/// vertex-buffer-drawing code.
#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    pub xyz: [f32; 3],
    pub col: [u8; 4],
}

/// Use positive values to expand the space between characters, and small negative
/// values (no smaller than -1.5) to contract the space between characters.
/// E.g. spacing = 1 adds one "pixel" of spacing between the characters.
/// spacing = -1 is reasonable but feels a bit too compact to me;
/// -0.5 is a reasonable compromise as long as
/// you're scaling the font up.
pub fn stb_easy_font_spacing(spacing: f32) {
    unsafe { sys::stb_easy_font_spacing_(spacing) };
}

/// Takes a string and returns the horizontal size
pub fn stb_easy_font_width(text: &CStr) -> i32 {
    unsafe { sys::stb_easy_font_width_(text.as_ptr() as *mut i8) }
}

/// Takes a string and returns the vertical size (which can vary if `text` has newlines)
pub fn stb_easy_font_height(text: &CStr) -> i32 {
    unsafe { sys::stb_easy_font_height_(text.as_ptr() as *mut i8) }
}

/// Takes a string (which can contain '\n') and fills out a vertex buffer with renderable data to
/// draw the string. Output data assumes increasing x is rightwards, increasing y is downwards.
/// The vertex data is divided into quads, i.e. there are four vertices in the vertex buffer for each quad.
/// If you pass in `None` for `color`, it becomes [255, 255, 255, 255].
/// Returns the number of quads.
/// If the buffer isn't large enough, it will truncate.
/// Expect it to use an average of ~270 bytes per character.
pub fn stb_easy_font_print(
    x: f32,
    y: f32,
    text: &CStr,
    color: Option<[u8; 4]>,
    buffer: &mut [Vertex],
) -> usize {
    let buffer_size = size_of::<Vertex>() * buffer.len();

    let color = if let Some(color) = color {
        color.as_ptr() as *mut u8
    } else {
        std::ptr::null_mut()
    };

    let quad_count = unsafe {
        sys::stb_easy_font_print_(
            x,
            y,
            text.as_ptr() as *mut i8,
            color,
            buffer.as_mut_ptr() as *mut std::ffi::c_void,
            buffer_size as i32,
        )
    };

    quad_count as _
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    #[test]
    fn stb_easy_font_width() {
        let abc = CString::new("abc").unwrap();
        assert_eq!(18, super::stb_easy_font_width(&abc));
    }

    #[test]
    fn stb_easy_font_height() {
        let abc = CString::new("ab\nc").unwrap();
        assert_eq!(24, super::stb_easy_font_height(&abc));
    }

    #[test]
    fn stb_easy_font_print() {
        let mut vertices = [super::Vertex::default(); 128];
        let a = CString::new("a").unwrap();
        let quad_count = super::stb_easy_font_print(1.0, 1.0, &a, None, &mut vertices);

        let vertex_count = quad_count * 4;

        assert_eq!(vertex_count, 16);
        for v in &vertices[..vertex_count] {
            assert_eq!(v.col, [255, 255, 255, 255]);
        }
    }

    #[test]
    fn stb_easy_font_print_empty() {
        let mut buffer = [super::Vertex::default(); 0];
        let a = CString::new("a").unwrap();
        let quad_count = super::stb_easy_font_print(1.0, 1.0, &a, None, &mut buffer);
        assert_eq!(0, quad_count);
    }
}
