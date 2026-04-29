const MAX_CH: u32 = 255u32;

struct Color {
    r: u32,
    g: u32,
    b: u32,
}

fn blend(a: Color, b: Color) -> Color {
    return Color { r: (a.r + b.r) / 2u32, g: (a.g + b.g) / 2u32, b: (a.b + b.b) / 2u32 };
}
