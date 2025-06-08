use std::arch::asm;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;
use std::sync::{Arc, Mutex};
use std::thread;

use pyo3::exceptions;
use pyo3::prelude::*;
use regex::Regex;

use crate::color::Color;
use crate::Direction;
use crate::RapidShifterIter;

type Shift = Arc<Mutex<Vec<String>>>;

const NONE: &str = "<None>";

macro_rules! lazy_check {
    ($param: expr, $func: expr) => {
        if $param.is_none() {
            $func;
        }
    };
}

fn magic(x: &str, y: &str) -> Ordering {
    let mut iter = zip(x.as_bytes(), y.as_bytes());
    let (mut key_x, mut key_y): (u8, u8);
    loop {
        if let Some((dx, dy)) = iter.next() {
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

/// RapidShifter
#[pyclass]
pub struct RapidShifter {
    #[pyo3(get)]
    desc: String,
    #[pyo3(get)]
    url: Option<String>,

    shifts: Option<Vec<String>>,
}

#[pymethods]
impl RapidShifter {
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

    pub fn __getitem__(&mut self, index: usize) -> PyResult<&String> {
        lazy_check!(self.shifts, self.process());

        if index >= self.shifts.as_ref().unwrap().len() {
            return Err(exceptions::PyIndexError::new_err(
                "arg `index` is out of range",
            ));
        }
        Ok(&self.shifts.as_ref().unwrap()[index])
    }

    pub fn process(&mut self) {
        let mut shifts = self.iter().collect::<Vec<String>>();
        shifts.sort_unstable_by(|x, y| magic(x, y));
        self.shifts = Some(shifts);
    }

    pub fn shifts(&mut self) -> &Vec<String> {
        lazy_check!(self.shifts, self.process());

        self.shifts.as_ref().unwrap()
    }

    pub fn show_line(&mut self, line: usize) -> PyResult<()> {
        lazy_check!(self.shifts, self.process());

        if line > self.shifts.as_ref().unwrap().len() {
            return Err(exceptions::PyIndexError::new_err(
                "arg `line` is out of range",
            ));
        }

        let line = &self.shifts.as_ref().unwrap()[line];

        let none = String::from("<None>");
        let url = self.url.as_ref().unwrap_or(&none);

        println!("{line} {url}");
        Ok(())
    }

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

    pub fn qshifts(&self, py: Python<'_>) -> Vec<String> {
        py.allow_threads(move || {
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

            /*
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

    fn show(&self) {}
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

const THREADS: usize = 16;

type UrlID = u64;

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

#[pyclass]
pub struct RapidShifterLines {
    item: Vec<Item>,
    shifts: Option<Vec<Item>>,

    urlmap: HashMap<UrlID, String>,
}

#[pymethods]
impl RapidShifterLines {
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

    pub fn process(&mut self, py: Python<'_>) {
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

    pub fn shifts(&mut self, py: Python<'_>) -> Vec<&String> {
        lazy_check!(self.shifts, self.process(py));

        self.shifts
            .as_ref()
            .unwrap()
            .iter()
            .map(|s| &s.desc)
            .collect()
    }

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
}
