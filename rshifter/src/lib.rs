use crate::shifter::{RapidShifter, RapidShifterLines};
use pyo3::prelude::*;

mod color;
mod shifter;

/// A Python module implemented in Rust.
#[pymodule]
fn rshifter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RapidShifter>()?;
    m.add_class::<RapidShifterLines>()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::RapidShifter;
    use crate::RapidShifterLines;

    use regex::Regex;

    #[test]
    fn test1() {
        let input: String = String::from("aaa bbb ccc ddd");
        let _ = RapidShifter::new(input).shifts();
    }

    #[test]
    fn test2() {
        let input: Vec<String> = [
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
