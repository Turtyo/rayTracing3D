#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

static BLACK: Color = Color {
    r: u8::MIN,
    g: u8::MIN,
    b: u8::MIN,
};
static WHITE: Color = Color {
    r: u8::MAX,
    g: u8::MAX,
    b: u8::MAX,
};
