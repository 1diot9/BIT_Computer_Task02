use std::arch::asm;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter::zip;
use std::sync::{Arc, Mutex};
use std::thread;

use pyo3::exceptions;
use pyo3::prelude::*;
use regex::Regex;

use crate::color::Color;

type Shift = Arc<Mutex<Vec<String>>>;

enum Direction {
    Left,
    Right,
}

#[derive(Clone)]
struct Item {
    desc: String,
    url: Option<String>,
}

impl Item {
    fn new(desc: String, url: Option<String>) -> Self {
        Item { desc, url }
    }
}

pub trait Shifter {
    fn show_all(verbose: bool);
}

fn maigc_sort(vector: &mut [String]) {
    vector.sort_unstable_by(|x, y| {
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
    });
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
        if self.shifts.is_none() {
            self.process();
        }

        if index >= self.shifts.as_ref().unwrap().len() {
            return Err(exceptions::PyIndexError::new_err(
                "arg `index` is out of range",
            ));
        }
        Ok(&self.shifts.as_ref().unwrap()[index])
    }

    pub fn process(&mut self) {
        let mut shifts = self.iter().collect::<Vec<String>>();
        maigc_sort(&mut shifts);
        self.shifts = Some(shifts);
    }

    pub fn shifts(&mut self) -> &Vec<String> {
        if self.shifts.is_none() {
            self.process();
        }
        self.shifts.as_ref().unwrap()
    }

    pub fn show_line(&mut self, line: usize) -> PyResult<()> {
        if self.shifts.is_none() {
            self.process();
        }

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
        if self.shifts.is_none() {
            self.process();
        }

        let shifts = self.shifts.as_ref().unwrap();

        let none = String::from("<None>");
        let url = self.url.as_ref().unwrap_or(&none);

        for (num, line) in shifts.iter().enumerate() {
            let (line, url) = if verbose {
                print!("{}: ", num + 1);
                (&Color::Blue.color(line), &Color::Yellow.color(url))
            } else {
                (line, url)
            };
            println!("{line} {url}");
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
            maigc_sort(&mut result);
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

#[pyclass]
pub struct RapidShifterLines {
    item: Vec<Item>,
    shifts: Option<Vec<String>>,
}

#[pymethods]
impl RapidShifterLines {
    #[new]
    pub fn new(item: Vec<String>) -> Self {
        let re = Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap();

        let item = item
            .iter()
            .map(|s| {
                let (desc, url) = s.trim().rsplit_once(' ').unwrap_or_default();

                if re.is_match(url) {
                    return Item::new(desc.to_string(), Some(url.to_string()));
                };

                Item::new(s.to_owned(), None)
            })
            .collect();

        RapidShifterLines { item, shifts: None }
    }

    pub fn process(&mut self) {}

    pub fn shifts(&self, py: Python<'_>) -> Vec<String> {
        py.allow_threads(move || {
            let result: Shift = Arc::new(Mutex::new(Vec::new()));

            for pieces in self.item.chunks(THREADS) {
                let mut handles = Vec::new();

                for piece in pieces {
                    let string = piece.desc.to_string();
                    let part = Arc::clone(&result);

                    handles.push(thread::spawn(move || {
                        let shifts = &mut RapidShifterIter::new(
                            string.split_ascii_whitespace().collect(),
                            None,
                            Direction::Left,
                        )
                        .collect();
                        part.lock().unwrap().append(shifts);
                    }));
                }

                for handle in handles {
                    handle.join().unwrap();
                }
            }

            let mut result = (*result.lock().unwrap()).to_owned();
            maigc_sort(&mut result);
            result
        })
    }

    pub fn show_all(&self) {}
}

struct RapidShifterIter<'a> {
    queue: VecDeque<&'a str>,
    length: usize,
    direction: Direction,
}

impl RapidShifterIter<'_> {
    fn new(input: Vec<&str>, length: Option<usize>, direction: Direction) -> RapidShifterIter {
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
