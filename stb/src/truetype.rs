//! This library processes TrueType files
//! - Parse files
//! - Extract glyph metrics
//! - Extract glyph shapes
//! - Render glyphs to one-channel bitmaps with antialiasing (box filter)
//! - Render glyphs to one-channel SDF bitmaps (signed-distance field/function)

use stb_sys as sys;
use std::ptr;

pub type FontData = [u8];

/// Holds the bitmap memory allocated by stb
/// This object will properly deallocate the memory with `stbtt_FreeBitmap` once dropped.
pub struct Bitmap {
    data: *mut u8,
    width: i32,
    height: i32,
    xoff: i32,
    yoff: i32,
}

impl Bitmap {
    /// Returns the width and height of the bitmap, which is stored left-to-right, top-to-bottom
    pub fn size(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    /// Returns the offset (xoff, yoff) it pixel space from the glyph origin to the top-left of the bitmap
    pub fn offset(&self) -> (i32, i32) {
        (self.xoff, self.yoff)
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe {
            sys::stbtt_FreeBitmap(self.data, ptr::null_mut());
        }
    }
}

pub struct Font<'a> {
    _data: &'a FontData,
    info: sys::stbtt_fontinfo,
}

impl Font<'_> {
    /// Each .ttf/.ttc file may have more than one font. Each font has a sequential
    /// index number starting from 0. Call this function to get the font offset for
    /// a given index.
    /// It returns `None` if the index is out of range. A regular .ttf
    /// file will only define one font and it always be at offset 0, so it will
    /// return `Some(0)` for index 0, and `None` for all other indices.
    pub fn font_offset_for_index(buffer: &FontData, index: i32) -> Option<u32> {
        let index = unsafe { sys::stbtt_GetFontOffsetForIndex(buffer.as_ptr(), index) };
        if index < 0 {
            None
        } else {
            Some(index as _)
        }
    }

    /// This function will determine the number of fonts in a font file. TrueType
    /// collection (.ttc) files may contain multiple fonts, while TrueType font
    /// (.ttf) files only contain one font. The number of fonts can be used for
    /// indexing with the previous function where the index is between zero and one
    /// less than the total fonts. If an error occurs, -1 is returned.
    pub fn number_of_fonts(buffer: &FontData) -> Option<u32> {
        let count = unsafe { sys::stbtt_GetNumberOfFonts(buffer.as_ptr()) };
        if count < 0 {
            None
        } else {
            Some(count as _)
        }
    }

    /// Load a font file from a memory buffer
    ///
    /// Given an `offset` into the file that defines a font, this function builds
    /// the necessary cached info for the rest of the system.
    pub fn new(data: &FontData, offset: i32) -> Option<Font> {
        let mut info = sys::stbtt_fontinfo::default();
        let ret = unsafe { sys::stbtt_InitFont(&mut info, data.as_ptr(), offset) };

        if ret == 0 {
            // stbtt_InitFont returns 0 if failed
            None
        } else {
            Some(Font { _data: data, info })
        }
    }

    /// Allocates a large-enough single-channel 8bpp bitmap and renders the
    /// specified character/glyph at the specified scale into it, with
    /// antialiasing.
    /// 0 is no coverage (transparent), 255 is fully covered (opaque).
    pub fn get_codepoint_bitmap(&self, scale_x: f32, scale_y: f32, codepoint: i32) -> Bitmap {
        let mut width = 0;
        let mut height = 0;
        let mut xoff = 0;
        let mut yoff = 0;

        let data = unsafe {
            sys::stbtt_GetCodepointBitmap(
                &self.info,
                scale_x,
                scale_y,
                codepoint,
                &mut width,
                &mut height,
                &mut xoff,
                &mut yoff,
            )
        };

        Bitmap {
            data,
            width,
            height,
            xoff,
            yoff,
        }
    }

    /// the same as `get_codepoint_bitmap`, but you can specify a subpixel shift for the character
    pub fn get_codepoint_bitmap_subpixel(
        &self,
        scale_x: f32,
        scale_y: f32,
        shift_x: f32,
        shift_y: f32,
        codepoint: i32,
    ) -> Bitmap {
        let mut width = 0;
        let mut height = 0;
        let mut xoff = 0;
        let mut yoff = 0;

        let data = unsafe {
            sys::stbtt_GetCodepointBitmapSubpixel(
                &self.info,
                scale_x,
                scale_y,
                shift_x,
                shift_y,
                codepoint,
                &mut width,
                &mut height,
                &mut xoff,
                &mut yoff,
            )
        };

        Bitmap {
            data,
            width,
            height,
            xoff,
            yoff,
        }
    }

    /// If you're going to perform multiple operations on the same character
    /// and you want a speed-up, call this function with the character you're
    /// going to process, then use glyph-based functions instead of the
    /// codepoint-based functions.
    /// Returns `None` if the character codepoint is not defined in the font.
    pub fn find_glyph_index(&self, unicode_codepoint: i32) -> Option<i32> {
        let ret = unsafe { sys::stbtt_FindGlyphIndex(&self.info, unicode_codepoint) };
        if ret == 0 {
            None
        } else {
            Some(ret)
        }
    }

    /// Computes a scale factor to produce a font whose "height" is `pixels` tall.
    ///
    /// Height is measured as the distance from the highest ascender to the lowest
    /// descender; in other words, it's equivalent to calling `stbtt_GetFontVMetrics`
    /// and computing: `scale = pixels / (ascent - descent)`
    ///
    /// so if you prefer to measure height by the ascent only, use a similar calculation.
    pub fn scale_for_pixel_height(&self, pixels: f32) -> f32 {
        unsafe { sys::stbtt_ScaleForPixelHeight(&self.info, pixels) }
    }

    /// Computes a scale factor to produce a font whose EM size is mapped to
    /// `pixels` tall.
    pub fn scale_for_mapping_em_to_pixels(&self, pixels: f32) -> f32 {
        unsafe { sys::stbtt_ScaleForMappingEmToPixels(&self.info, pixels) }
    }
}
