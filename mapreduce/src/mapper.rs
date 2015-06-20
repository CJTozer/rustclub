use std::sync::mpsc::{Receiver, Sender};

pub trait Mapper<Data, Output> {
    fn map(data_in: Receiver<Data>,
           data_out: Sender<Output>);
}

pub struct Incrementer;
impl Mapper<i32, i32> for Incrementer {
    fn map(data_in: Receiver<i32>,
           data_out: Sender<i32>) {
        loop {
            match data_in.recv() {
                Ok(x) => data_out.send(x + 1).unwrap(),
                Err(_) => break, // Assume channel closed
            }
        }

        // Close the output channel
        drop(data_out);
    }
}
