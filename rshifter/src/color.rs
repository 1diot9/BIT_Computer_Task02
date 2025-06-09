pub const END: &str = "\x1b[0m";

#[derive(Debug)]
#[allow(unused)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
}

impl Color {
    pub fn color(&self, string: &str) -> String {
        let clr = match self {
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Purple => "35",
            Color::Cyan => "36",
        };

        format!("\x1b[{clr}m{string}{END}")
    }
}
