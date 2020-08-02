use stb::easy_font::{stb_easy_font_print, Vertex};
use std::ffi::CString;

#[allow(clippy::identity_op)]

fn main() {
    let text = CString::new("Hello world").unwrap();
    let mut buffer = [Vertex::default(); 1024];

    let quad_count = stb_easy_font_print(1.0, 1.0, &text, None, &mut buffer);

    let vertex_count = quad_count * 4;
    let vertices = &buffer[..vertex_count];

    let index_count = quad_count * 6;

    // If your API doesn't support quad rendering, build a reusable index list that allows you to
    // render quads as indexed triangles.

    let mut indices = vec![0; index_count as _];

    for i in 0..index_count / 6 {
        indices[6 * i + 0] = 4 * i as u32 + 0;
        indices[6 * i + 1] = 4 * i as u32 + 1;
        indices[6 * i + 2] = 4 * i as u32 + 2;

        indices[6 * i + 3] = 4 * i as u32 + 0;
        indices[6 * i + 4] = 4 * i as u32 + 2;
        indices[6 * i + 5] = 4 * i as u32 + 3;
    }

    println!("Quad count: {}", quad_count);
    println!("Vertex count: {}", vertex_count);
    println!("Index count: {}", index_count);

    println!("Vertices: {:?}", vertices);
    println!("Indices: {:?}", indices);
}
