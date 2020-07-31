#include "../../vendor/stb/stb_easy_font.h"

// Linker won't generate symbols for static functions defined in the header file, so we add these wrappers.
// See https://users.rust-lang.org/t/ffi-and-static-functions-in-headers/9995

int stb_easy_font_print_(float x, float y, char *text, unsigned char color[4], void *vertex_buffer, int vbuf_size) {
    return stb_easy_font_print(x, y, text, color, vertex_buffer, vbuf_size);
}

int stb_easy_font_draw_segs_(float x, float y, unsigned char *segs, int num_segs, int vertical, stb_easy_font_color c, char *vbuf, int vbuf_size, int offset) {
    return stb_easy_font_draw_segs(x, y, segs, num_segs, vertical, c, vbuf, vbuf_size, offset);
}

int stb_easy_font_width_(char *text) {
    return stb_easy_font_width(text);
}

int stb_easy_font_height_(char *text) {
    return stb_easy_font_height(text);
}

void stb_easy_font_spacing_(float spacing) {
    stb_easy_font_spacing(spacing);
}
