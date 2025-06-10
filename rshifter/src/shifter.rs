//! 循环移位器[`RapidShifter`]与[`RapidShifterLines`]的实现模块
//! 提供移位产生/搜索等一系列功能

use std::arch::asm;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;
use std::sync::{Arc, Mutex};
use std::thread;

use pyo3::create_exception;
use pyo3::exceptions;
use pyo3::prelude::*;
use regex::Regex;

use crate::color::Color;
use crate::Direction;
use crate::RapidShifterIter;

// TODO: 增加搜索高亮显示功能

type Shift = Arc<Mutex<Vec<String>>>;

/// 若未匹配到URL，使用该字符串代替
const NONE: &str = "<None>";

create_exception!(rshifter, PyRegexSyntaxError, pyo3::exceptions::PyException);
create_exception!(
    rshifter,
    PyRegexCompiledTooBigError,
    pyo3::exceptions::PyException
);

macro_rules! lazy_check {
    ($param: expr, $func: expr) => {
        if $param.is_none() {
            $func;
        }
    };
}

/// 移位序列排序函数
/// 可以按"a > A > b > B > c > C ..."顺序进行排序
/// 原理参见Python版本注释
fn magic(x: &str, y: &str) -> Ordering {
    let mut iter = zip(x.as_bytes(), y.as_bytes());
    let (mut key_x, mut key_y): (u8, u8);
    loop {
        if let Some((dx, dy)) = iter.next() {
            // HACK: That works, see also in python version
            // SAFETY: only read dx & dy
            unsafe {
                asm!(
                    "mov r10b, BYTE PTR [r8]",
                    "mov r11b, BYTE PTR [r9]",
                    "rol r10b, 3", "rol r11b, 3", "xor r10b, 0x1", "xor r11b, 0x1",
                    in("r8") dx,
                    in("r9") dy,
                    out("r10b") key_x,
                    out("r11b") key_y,
                );
            }

            match key_x.cmp(&key_y) {
                Ordering::Less => break Ordering::Less,
                Ordering::Greater => break Ordering::Greater,
                Ordering::Equal => continue,
            }
        } else {
            break y.len().cmp(&x.len());
        };
    }
}

/// 快速移位序列结构体`RapidShifter`
/// 存储描述`desc`，URL`url`与排序后移位序列`shifts`
///
/// 使用[`lazy_check!`]宏进行惰性处理，在需要时才会进行移位排序，产生开销
/// 也可以提前调用方法[`RapidShifter::process`]来产生所有移位序列
#[pyclass]
#[derive(Debug)]
pub struct RapidShifter {
    #[pyo3(get)]
    desc: String,
    #[pyo3(get)]
    url: Option<String>,

    shifts: Option<Vec<String>>,
}

// TODO: use mark-based sort and optimize search
// add `fn search_show()`

#[pymethods]
impl RapidShifter {
    /// 初始化函数
    /// 参数`desc`为给定字符串
    /// 使用正则表达式尝试匹配URL，若匹配失败则设置URL为[`None`]
    /// 仅匹配最后一个单词块（以' '作为分隔符）
    #[new]
    #[pyo3(signature = (desc, /))]
    pub fn new(desc: String) -> Self {
        let (string, url) = desc.trim().rsplit_once(' ').unwrap_or_default();

        let re = Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap();

        if re.is_match(url) {
            return RapidShifter {
                desc: string.to_string(),
                shifts: None,
                url: Some(url.to_string()),
            };
        };

        RapidShifter {
            desc,
            shifts: None,
            url: None,
        }
    }

    /// 移位排序处理函数
    /// 调用仅会生成所有移位序列并排序，不会返回值
    /// 需要移位序列参见函数[`RapidShifter::\_\_getitem\_\_`]或[`RapidShifter::shifts`]
    pub fn process(&mut self) {
        let mut shifts = self.iter().collect::<Vec<String>>();
        shifts.sort_unstable_by(|x, y| magic(x, y));
        self.shifts = Some(shifts);
    }

    /// 得到特定的循环序列
    pub fn __getitem__(&mut self, index: usize) -> PyResult<String> {
        lazy_check!(self.shifts, self.process());

        if index >= self.shifts.as_ref().unwrap().len() {
            return Err(exceptions::PyIndexError::new_err(
                "arg `index` is out of range",
            ));
        }

        let res = format!(
            "{} {}",
            &self.shifts.as_ref().unwrap()[index],
            &self.url.as_ref().map_or(NONE, |s| s.as_str())
        );

        Ok(res)
    }

    /// 得到所有的循环序列
    pub fn shifts(&mut self) -> &Vec<String> {
        lazy_check!(self.shifts, self.process());

        self.shifts.as_ref().unwrap()
    }

    /// 展示特定列
    pub fn show_line(&mut self, line: usize) -> PyResult<()> {
        lazy_check!(self.shifts, self.process());

        if line > self.shifts.as_ref().unwrap().len() {
            return Err(exceptions::PyIndexError::new_err(
                "arg `line` is out of range",
            ));
        }

        let line = &self.shifts.as_ref().unwrap()[line];

        let url = self.url.as_ref().map_or(NONE, |s| s.as_str());

        println!("{line} {url}");
        Ok(())
    }

    /// 展示所有列
    /// 参数`verbose`为是否详细展示
    #[pyo3(signature = (verbose=false))]
    pub fn show_all(&mut self, verbose: bool) {
        lazy_check!(self.shifts, self.process());

        let shifts = self.shifts.as_ref().unwrap();

        let url = self.url.as_ref().map_or(NONE, |s| s.as_str());

        for (num, line) in shifts.iter().enumerate() {
            if verbose {
                print!("{}", Color::Purple.color(&format!("[{:0>2}] ", num + 1)));
                println!("{} {}", Color::Blue.color(line), Color::Yellow.color(url));
            } else {
                println!("{line} {url}");
            };
        }
    }

    /// 搜索特定字符串
    /// 参数`all`设置搜索内容是否包括URL
    #[pyo3(signature = (pat, all=false))]
    pub fn search(&mut self, pat: String, all: bool) -> Option<Vec<usize>> {
        lazy_check!(self.shifts, self.process());

        let shifts = self.shifts.as_ref().unwrap();
        let desc_len = self.desc.len();

        if (all && self.url.as_ref().unwrap_or(&String::new()).contains(&pat))
            || (!pat.contains(" ") && self.desc.contains(&pat))
        {
            return Some((0..shifts.len()).collect());
        }

        let pat_len = pat.len();

        let url = if all {
            &format!(" {}", self.url.as_ref().map_or(NONE, |s| s.as_str()))
        } else {
            ""
        };

        if (!all && desc_len <= pat_len) || (all && desc_len + url.len() < pat_len) {
            return None;
        }

        let mut res: Vec<usize> = Vec::new();

        for (index, shift) in shifts.iter().enumerate() {
            if format!("{shift}{url}").contains(&pat) {
                res.push(index);
            }
        }

        if res.is_empty() {
            None
        } else {
            Some(res)
        }

        // TODO: Fix this
        /*
        let double = format!("{desc} {desc}", desc = &self.desc);

        let begin: usize;
        let end: usize;

        dbg!(&double);
        if let Some(n) = double.find(&pat) {
            let start = &double[..n];
            let last = &double[(n + pat_len)..];

            let start_cnt = start.matches(' ').count();
            let pat_cnt = pat.matches(' ').count();

            begin = if start.ends_with(' ') {
                start_cnt
            } else {
                start_cnt - 1
            };

            end = if last.starts_with(' ') {
                begin + pat_cnt
            } else {
                begin + pat_cnt + 1
            };

            dbg!(&start);
            dbg!(&last);
            dbg!(&begin);
            dbg!(&end);
            return Some((begin..end).collect());
        }
        */
    }

    /// 通过正则表达式搜索特定字符串
    /// 参数`all`设置搜索内容是否包括URL
    #[pyo3(signature = (re, all=false))]
    pub fn regex_search(&mut self, re: &str, all: bool) -> PyResult<Option<Vec<usize>>> {
        lazy_check!(self.shifts, self.process());

        let re = match Regex::new(re) {
            Ok(re) => re,
            Err(err) => match err {
                regex::Error::CompiledTooBig(n) => {
                    return Err(PyRegexCompiledTooBigError::new_err(format!(
                        "Arg `{re}`(size: {n}) is too big",
                    )))
                }
                regex::Error::Syntax(_) => {
                    return Err(PyRegexSyntaxError::new_err(format!(
                        "Arg `{re}` is not a valid Regular Expression",
                    )))
                }
                _ => unreachable!(),
            },
        };

        let mut res: Vec<usize> = Vec::new();
        let shifts = self.shifts.as_ref().unwrap();

        let url = if all {
            &format!(" {}", self.url.as_ref().map_or(NONE, |s| s.as_str()))
        } else {
            ""
        };

        for (index, shift) in shifts.iter().enumerate() {
            if re.is_match(&format!("{shift}{url}")) {
                res.push(index);
            }
        }

        if res.is_empty() {
            Ok(None)
        } else {
            Ok(Some(res))
        }
    }

    /// 并发生成循环移位序列
    /// 理论上应该更快，但是比Python还慢
    /// 目前废弃(deprecated)处理
    #[deprecated]
    pub fn qshifts(&self, py: Python<'_>) -> Vec<String> {
        py.allow_threads(move || {
            // PERF: Use concurency to "optimize" it
            // But I find it even slowly than Python ??? why???
            let words: Vec<String> = self
                .desc
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect();

            let result: Shift = Arc::new(Mutex::new(Vec::from([self.desc.clone()])));

            let words_clone = words.clone();

            let length = words.len();
            let left = length / 2;
            let right = length - left - 1;

            let mut handles = Vec::new();

            let partl = Arc::clone(&result);
            let partr = Arc::clone(&result);

            handles.push(thread::spawn(move || {
                //let mut words: Vec<&str> = input.split_ascii_whitespace().collect();
                //words.rotate_left(length / threads * i);
                let shifts = &mut RapidShifterIter::new(
                    words.iter().map(|s| s.as_str()).collect(),
                    Some(left),
                    Direction::Left,
                )
                .collect();

                partl.lock().unwrap().append(shifts);
            }));

            handles.push(thread::spawn(move || {
                //let mut words: Vec<&str> = input.split_ascii_whitespace().collect();
                //words.rotate_left(length / threads * i);
                let shifts = &mut RapidShifterIter::new(
                    words_clone.iter().map(|s| s.as_str()).collect(),
                    Some(right),
                    Direction::Right,
                )
                .collect();

                partr.lock().unwrap().append(shifts);
            }));

            /* PERF: The code below is slower...

            let result: Shift = Arc::new(Mutex::new(Vec::from([self.input.clone()])));
            let mut words: Vec<String> = self
                .input
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect();

            let length = words.len();

            if threads > length {
                threads = length;
            }

            let step = length / threads;

            let mut handles = Vec::new();

            for _ in 0..threads {
                let part = Arc::clone(&result);
                //let input = self.input.clone();
                let words_clone = words.clone();

                handles.push(thread::spawn(move || {
                    //let mut words: Vec<&str> = input.split_ascii_whitespace().collect();
                    //words.rotate_left(length / threads * i);
                    let shifts = &mut RapidShifterIter::new(
                        words_clone.iter().map(|s| s.as_str()).collect(),
                        Some(step),
                        Direction::Left,
                    )
                    .collect();

                    part.lock().unwrap().append(shifts);
                }));

                words.rotate_left(step);
            }

            let part = Arc::clone(&result);
            let res = length % threads;
            words.rotate_left(step);

            let shifts = &mut RapidShifterIter::new(
                words.iter().map(|s| s.as_str()).collect(),
                Some(res),
                Direction::Left,
            )
            .collect();
            part.lock().unwrap().append(shifts);
            */

            for handle in handles {
                handle.join().unwrap();
            }

            let mut result = (*result.lock().unwrap()).to_owned();
            result.sort_unstable_by(|x, y| magic(x, y));
            result
        })
    }
}

impl RapidShifter {
    fn iter(&self) -> RapidShifterIter {
        RapidShifterIter::new(
            self.desc.split_ascii_whitespace().collect(),
            None,
            Direction::Left,
        )
    }
}

/// [`RapidShifterLines`]并发设置的最大线程数
const THREADS: usize = 16;

type UrlID = u64;

/// 接受的行参数类型
/// 字段`url_id`通过[`RapidShifterLines::urlmap`]对应着一个URL
#[derive(Clone)]
struct Item {
    desc: String,
    url_id: Option<UrlID>,
}

impl Item {
    fn new(desc: String, url: Option<UrlID>) -> Self {
        Item { desc, url_id: url }
    }
}

/// 多行循环移位结构体`RapidShifterLines`
///
/// 通过哈希([`HashMap<UrlID, String>`])存储URL与每一行移位序列的关系
/// 通过宏[`lazy_check!`]惰性排序
/// 效果同[`RapidShifter`]
///
/// > 注意：本结构体目前不支持`merge`参数，默认行为是合并操作
#[pyclass]
pub struct RapidShifterLines {
    item: Vec<Item>,
    shifts: Option<Vec<Item>>,

    urlmap: HashMap<UrlID, String>,
}

#[pymethods]
impl RapidShifterLines {
    /// 初始化函数
    /// 参数`item`为给定字符串
    /// 使用正则表达式尝试匹配URL，若匹配失败则设置URL为[`None`]
    /// 仅匹配最后一个单词块（以' '作为分隔符）
    #[new]
    pub fn new(item: Vec<String>) -> Self {
        let re = Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap();

        let mut urlmap = HashMap::new();
        let mut id = 0u64;

        let item = item
            .iter()
            .map(|s| {
                let (desc, url) = s.trim().rsplit_once(' ').unwrap_or_default();

                if re.is_match(url) {
                    id += 1;
                    urlmap.insert(id, url.to_string());
                    return Item::new(desc.to_string(), Some(id));
                };

                Item::new(s.to_owned(), None)
            })
            .collect();

        RapidShifterLines {
            item,
            shifts: None,
            urlmap,
        }
    }

    /// 移位排序处理函数
    ///
    /// 通过**并发**来加速移位过程，可以同时运行多个移位迭代器
    /// 调用仅会生成所有移位序列并排序，不会返回值
    /// 需要移位序列参见函数[`RapidShifterLines::\_\_getitem\_\_`]或[`RapidShifterLines::shifts`]
    pub fn process(&mut self, py: Python<'_>) {
        // PERF: Use concurency to optimize it
        // Max threads is set to 16
        py.allow_threads(move || {
            let result: Arc<Mutex<Vec<Item>>> = Arc::new(Mutex::new(Vec::new()));

            for pieces in self.item.chunks(THREADS) {
                let mut handles = Vec::new();

                for piece in pieces {
                    let string = piece.desc.to_string();
                    let url_id = piece.url_id;

                    let part = Arc::clone(&result);

                    handles.push(thread::spawn(move || {
                        let shifts = &mut RapidShifterIter::new(
                            string.split_ascii_whitespace().collect(),
                            None,
                            Direction::Left,
                        )
                        .map(|s| Item::new(s, url_id))
                        .collect();
                        part.lock().unwrap().append(shifts);
                    }));
                }

                for handle in handles {
                    handle.join().unwrap();
                }
            }

            let mut result = (*result.lock().unwrap()).to_owned();
            result.sort_unstable_by(|x, y| magic(&x.desc, &y.desc));
            self.shifts = Some(result);
        })
    }

    /// 得到特定的循环序列
    pub fn __getitem__(&mut self, py: Python<'_>, index: usize) -> PyResult<String> {
        lazy_check!(self.shifts, self.process(py));

        if index >= self.shifts.as_ref().unwrap().len() {
            return Err(exceptions::PyIndexError::new_err(
                "arg `index` is out of range",
            ));
        }

        let shift = &self.shifts.as_ref().unwrap()[index];

        let url = match shift.url_id {
            Some(id) => self.urlmap.get(&id).unwrap(),
            None => NONE,
        };

        let res = format!("{} {}", &shift.desc, url);

        Ok(res)
    }

    /// 得到所有的循环序列
    pub fn shifts(&mut self, py: Python<'_>) -> Vec<&String> {
        lazy_check!(self.shifts, self.process(py));

        self.shifts
            .as_ref()
            .unwrap()
            .iter()
            .map(|s| &s.desc)
            .collect()
    }

    /// 展示特定列
    pub fn show_line(&mut self, py: Python<'_>, line: usize) -> PyResult<()> {
        lazy_check!(self.shifts, self.process(py));

        if line > self.shifts.as_ref().unwrap().len() {
            return Err(exceptions::PyIndexError::new_err(
                "arg `line` is out of range",
            ));
        }

        let line = &self.shifts.as_ref().unwrap()[line];

        let desc = &line.desc;
        let url = match line.url_id {
            Some(id) => self.urlmap.get(&id).unwrap(),
            None => NONE,
        };

        println!("{desc} {url}");
        Ok(())
    }

    /// 展示所有列
    /// 参数`verbose`为是否详细展示
    #[pyo3(signature = (verbose=false))]
    pub fn show_all(&mut self, py: Python<'_>, verbose: bool) {
        lazy_check!(self.shifts, self.process(py));

        let shifts = self.shifts.as_ref().unwrap();

        for (num, line) in shifts.iter().enumerate() {
            let desc = &line.desc;
            let url = match line.url_id {
                Some(id) => self.urlmap.get(&id).unwrap(),
                None => NONE,
            };

            if verbose {
                print!("{}", Color::Purple.color(&format!("[{:0>2}] ", num + 1)));
                println!("{} {}", Color::Blue.color(desc), Color::Yellow.color(url));
            } else {
                println!("{desc} {url}");
            };
        }
    }

    /// 搜索特定字符串
    /// 参数`all`设置搜索内容是否包括URL
    #[pyo3(signature = (pat, all=false))]
    pub fn search(&mut self, py: Python<'_>, pat: String, all: bool) -> Option<Vec<usize>> {
        // TODO: optimize this
        //
        lazy_check!(self.shifts, self.process(py));

        let mut res: Vec<usize> = Vec::new();

        for (index, line) in self.shifts.as_ref().unwrap().iter().enumerate() {
            let desc = &line.desc;
            if all {
                let url = match line.url_id {
                    Some(id) => self.urlmap.get(&id).unwrap(),
                    None => NONE,
                };

                if format!("{} {}", desc, url).contains(&pat) {
                    res.push(index);
                }
            } else if desc.contains(&pat) {
                res.push(index);
            }
        }

        if res.is_empty() {
            None
        } else {
            Some(res)
        }

        /*
        let double = format!("{desc} {desc}", desc = &self.desc);

        let begin: usize;
        let end: usize;

        dbg!(&double);
        if let Some(n) = double.find(&pat) {
            let start = &double[..n];
            let last = &double[(n + pat_len)..];

            let start_cnt = start.matches(' ').count();
            let pat_cnt = pat.matches(' ').count();

            begin = if start.ends_with(' ') {
                start_cnt
            } else {
                start_cnt - 1
            };

            end = if last.starts_with(' ') {
                begin + pat_cnt
            } else {
                begin + pat_cnt + 1
            };

            dbg!(&start);
            dbg!(&last);
            dbg!(&begin);
            dbg!(&end);
            return Some((begin..end).collect());
        }
        */

        // 01234567
        //
        // A B C D E F A B C D E F
        //  "B C D"
        //   n..(n + len - 1)
        // "A " =>  1 => 0
        //  B C D => ban 2 3 4 => 2
        // " E F ..." 5 ..
        //
        // A B C D E F A B C D E F
        //       " E F A B"
        // "A B C D" => 3
        // " E F A B" => none
    }

    /// 通过正则表达式搜索特定字符串
    /// 参数`all`设置搜索内容是否包括URL
    #[pyo3(signature = (re, all=false))]
    pub fn regex_search(
        &mut self,
        py: Python<'_>,
        re: &str,
        all: bool,
    ) -> PyResult<Option<Vec<usize>>> {
        lazy_check!(self.shifts, self.process(py));

        let re = match Regex::new(re) {
            Ok(re) => re,
            Err(err) => match err {
                regex::Error::CompiledTooBig(n) => {
                    return Err(PyRegexCompiledTooBigError::new_err(format!(
                        "Arg `{re}`(size: {n}) is too big",
                    )))
                }
                regex::Error::Syntax(_) => {
                    return Err(PyRegexSyntaxError::new_err(format!(
                        "Arg `{re}` is not a valid Regular Expression",
                    )))
                }
                _ => unreachable!(),
            },
        };

        let mut res: Vec<usize> = Vec::new();

        for (index, line) in self.shifts.as_ref().unwrap().iter().enumerate() {
            let desc = &line.desc;
            if all {
                let url = match line.url_id {
                    Some(id) => self.urlmap.get(&id).unwrap(),
                    None => NONE,
                };

                if re.is_match(&format!("{} {}", desc, url)) {
                    res.push(index);
                }
            } else if re.is_match(desc) {
                res.push(index);
            }
        }

        if res.is_empty() {
            Ok(None)
        } else {
            Ok(Some(res))
        }
    }
}
