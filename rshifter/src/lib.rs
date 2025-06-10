//! 模块rsifter的lib包
//!
//! 提供迭代器[`RapidShifterIter`]及其实现
//!
//! 注册Python类[`RapidShifter`]和[`RapidShifterLines`]，处于Python模块`rshifter`下
//! 将Rust结构体(`struct`)注册为Python类(`class`)
//!
//! # Example:
//!
//! 以下为Python示例代码
//! ```python
//! from rshifter import RapidShifter, RapidShifterLines
//!
//! tst1 = RapidShifter("aa aA ab aB ap aP")
//! res1 = [
//!     "aa aA ab aB ap aP",
//!     "aA ab aB ap aP aa",
//!     "ab aB ap aP aa aA",
//!     "aB ap aP aa aA ab",
//!     "ap aP aa aA ab aB",
//!     "aP aa aA ab aB ap",
//! ]
//! assert tst1.shifts() == res1
//!
//! tst2 = QuickShifterLines(
//!     ["A a B b", "Another yet new string",
//!     "Once upon a time", "It is my shift now"],
//! )
//!
//! res2 = [
//!     "a B b A",
//!     "A a B b",
//!     "b A a B",
//!     "B b A a",
//! ]
//! assert tst2[0] == res2
//! ```
use crate::shifter::{RapidShifter, RapidShifterLines};
use pyo3::prelude::*;
use std::collections::VecDeque;

pub mod color;
pub mod shifter;

/// 移位方向枚举
/// 用于迭代器[`RapidShifterIter`]，来确定移位的方向
pub enum Direction {
    /// 左方向，使用方法[`VecDeque::rotate_left`]
    Left,
    /// 右方向，使用方法[`VecDeque::rotate_right`]
    Right,
}

/// 迭代器`RapidShifterIter`
/// 用于产生所有的移位序列，在结构体[`RapidShifter`]和[`RapidShifterLines`]中使用
///
/// 通过双端队列[`VecDeque`]来高效操作，`direction`用来确定移位方向
/// `direction`类型为[`Direction`]，有两种选择
///
/// > 当`length`与`queue`的长度一样时，即使方向不同，产生的移位序列是一样的
pub struct RapidShifterIter<'a> {
    queue: VecDeque<&'a str>,
    length: usize,
    direction: Direction,
}

impl RapidShifterIter<'_> {
    /// 初始化迭代器
    /// `input`类型为[`Vec`]，会自动转换为双端队列[`VecDeque`]
    /// `length`类型为[`Option<usize>`]，当值为[`None`]时，产生的移位序列内容和方向无关
    /// `direction`类型为[`Direction`]，其值用于决定采用方法[`VecDeque::rotate_left`]还是[`VecDeque::rotate_right`]进行移位
    pub fn new(input: Vec<&str>, length: Option<usize>, direction: Direction) -> RapidShifterIter {
        let length = length.unwrap_or(input.len());
        RapidShifterIter {
            queue: VecDeque::from(input),
            length,
            direction,
        }
    }
}

impl Iterator for RapidShifterIter<'_> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            return None;
        }

        match self.direction {
            Direction::Left => self.queue.rotate_left(1),
            Direction::Right => self.queue.rotate_right(1),
        };

        self.length -= 1;

        Some(self.queue.make_contiguous().join(" "))
    }
}

#[pymodule]
fn rshifter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RapidShifter>()?;
    m.add_class::<RapidShifterLines>()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::RapidShifter;
    //use crate::RapidShifterLines;

    use regex::Regex;

    #[test]
    fn test1() {
        let input: String = String::from("aaa bbb ccc ddd");
        let _ = RapidShifter::new(input).shifts();
    }

    #[test]
    fn test2() {
        let _: Vec<String> = [
            "aaa bbb ccc ddd",
            "a A p p b B a W R P",
            "A simple test sentence",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        //let _ = RapidShifterLines::new(input).shifts();
    }

    #[test]
    fn test3() {
        let input = String::from("aaa bbb ccc ddd");

        let tst = RapidShifter::new(input).shifts().to_owned();
        assert_eq!(
            tst,
            [
                "aaa bbb ccc ddd",
                "bbb ccc ddd aaa",
                "ccc ddd aaa bbb",
                "ddd aaa bbb ccc"
            ]
        )
    }

    #[test]
    fn test4() {
        let input = String::from("aa aA ab aB ap aP");

        let tst = RapidShifter::new(input).shifts().to_owned();
        let res = [
            "aa aA ab aB ap aP",
            "aA ab aB ap aP aa",
            "ab aB ap aP aa aA",
            "aB ap aP aa aA ab",
            "ap aP aa aA ab aB",
            "aP aa aA ab aB ap",
        ];

        assert_eq!(tst, res);
    }

    #[test]
    fn test_reg() {
        let re = Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap();

        let tst = r"http://www.baidu.com";
        assert!(re.is_match(tst));

        assert!(re.is_match(r"http://www.1337.net"));
    }
}
