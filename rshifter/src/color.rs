//! 控制台色彩模块
//!
//! 提供颜色枚举[`Color`]和改变前景色的函数[`Color::color`]
//! 需要终端支持彩色

/// 取消所有的输出色彩/格式
/// 在函数[`Color::color`]中使用，追加在改变颜色字符串的末尾
pub const END: &str = "\x1b[0m";

/// 颜色枚举
/// 目前仅支持六种颜色，枚举值/顺序参见ANSI转义控制字符
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
    /// 将`string`前景色改变为自身枚举对应的颜色
    /// 原理参见ANSI转义控制字符
    ///
    /// # Example:
    ///
    /// ```rust
    /// let red = Color::Red;
    ///
    /// println!("{}", Color::Geeen.color("This text will be green"));
    /// println!("{}", red.color("This text will be red"));
    /// ```
    pub fn color(&self, string: &str) -> String {
        // HACK: That's crazy but it works!
        let clr = (*self).clone() as usize;

        format!("\x1b[{clr}m{string}{END}")
    }
}
