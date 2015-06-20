use std::sync::mpsc::{Receiver, Sender};
use std::collections::HashMap;

pub trait Mapper<Data, Output> {
    fn new(data_in: Receiver<Data>,
           data_out: Sender<Output>) -> Self;
    fn map(&self);
}

/// Simple mapper that sums numbers
pub struct Summer {
    data_in: Receiver<i32>,
    data_out: Sender<i32>,
}
impl Mapper<i32, i32> for Summer {
    fn new(data_in: Receiver<i32>,
           data_out: Sender<i32>) -> Summer {
        Summer { data_in: data_in, data_out: data_out }
    }

    fn map(&self) {
        let mut total = 0;
        loop {
            match self.data_in.recv() {
                Ok(x) => total += x,
                Err(_) => break, // Assume channel closed
            }
        }

        // Send the result and close the output channel
        self.data_out.send(total).unwrap();
        drop(self.data_out);
    }
}

pub struct WordCounter {
    data_in: Receiver<String>,
    data_out: Sender<HashMap<String, i32>>,
    counts: HashMap<String, i32>,
}
impl Mapper<String, HashMap<String, i32>> for WordCounter {
    fn new(data_in: Receiver<String>,
           data_out: Sender<HashMap<String, i32>>) -> WordCounter {
        WordCounter {
            data_in: data_in,
            data_out: data_out,
            counts: HashMap::new(),
        }
    }

    fn map(&self) {
        loop {
            match self.data_in.recv() {
                Ok(x) => {
                    self.count_words(x);
                },
                Err(_) => break, // Assume channel closed
            }
        }

        // Send the result and close the output channel
        self.data_out.send(self.counts).unwrap();
        drop(self.data_out);
    }
}
impl WordCounter {
    fn count_words(&self, s: String) {
        for ss in s.split(" ") {
            match self.counts.get(ss) {
                Some(x) => self.counts.insert(ss, x+1),
                _ => self.counts.insert(ss, 1),
            }
        }
    }
}

