pub const END: &str = "\x1b[0m";

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum Color {
    /// 红色
    Red = 31,
    /// 绿色
    Green,
    /// 黄色
    Yellow,
    /// 蓝色
    Blue,
    /// 紫色
    Purple,
    /// 青色
    Cyan,
}

impl Color {
    pub fn color(&self, string: &str) -> String {
        // HACK: That's crazy but it works!
        let clr = (*self).clone() as usize;

        format!("\x1b[{clr}m{string}{END}")
    }
}
