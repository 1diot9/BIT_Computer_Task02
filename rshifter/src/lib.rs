use std::arch::asm;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter::zip;
use std::sync::{Arc, Mutex};
use std::thread;

use pyo3::prelude::*;

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
                        out("r11b") key_x,
                        out("r10b") key_y,
                    );
                }

                match key_y.cmp(&key_x) {
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

/// QuickShifter
#[pyclass]
struct RapidShifter {
    #[pyo3(get)]
    input: String,
}

#[pymethods]
impl RapidShifter {
    #[new]
    fn new(input: String) -> Self {
        RapidShifter { input }
    }

    fn shifts(&self) -> Vec<String> {
        let mut res = self.iter().collect::<Vec<String>>();
        maigc_sort(&mut res);
        res
    }

    fn show(&self) {}
}

impl RapidShifter {
    fn iter(&self) -> RapidShifterIter {
        RapidShifterIter::new(self.input.split_ascii_whitespace().collect())
    }
}

const THREADS: usize = 8;

#[pyclass]
struct RapidShifterLines {
    #[pyo3(get)]
    input: Vec<String>,
}

#[pymethods]
impl RapidShifterLines {
    #[new]
    fn new(input: Vec<String>) -> Self {
        RapidShifterLines { input }
    }

    fn shifts(&self) -> Vec<String> {
        let arc_result: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::from(Vec::new()));

        for pieces in self.input.chunks(THREADS) {
            let mut handles = Vec::new();

            for piece in pieces {
                let string = piece.to_string();
                let part = Arc::clone(&arc_result);

                handles.push(thread::spawn(move || {
                    let shifts =
                        &mut RapidShifterIter::new(string.split_ascii_whitespace().collect())
                            .collect();
                    let mut list = part.lock().unwrap();
                    (*list).append(shifts)
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }
        }

        let mut result = (*arc_result.lock().unwrap()).clone();
        maigc_sort(&mut result);
        result
    }
}

struct RapidShifterIter<'a> {
    queue: VecDeque<&'a str>,
    count: usize,
    length: usize,
}

impl RapidShifterIter<'_> {
    fn new(input: Vec<&str>) -> RapidShifterIter {
        let length = input.len();
        RapidShifterIter {
            queue: VecDeque::from(input),
            count: 0,
            length,
        }
    }
}

impl Iterator for RapidShifterIter<'_> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.length {
            return None;
        }

        self.queue.rotate_left(1);

        self.count += 1;

        Some(self.queue.make_contiguous().join(" "))
    }
}

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

    #[test]
    fn test1() {
        let input: String = String::from("aaa bbb ccc ddd");
        let _ = RapidShifter::new(input).shifts();
    }

    #[test]
    fn test2() {
        let input = [
            "aaa bbb ccc ddd",
            "a A p p b B a W R P",
            "A simple test sentence",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let _ = RapidShifterLines::new(input).shifts();
    }
}
