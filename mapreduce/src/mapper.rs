use std::sync::mpsc::{Receiver, Sender};

pub trait Mapper<Data, Output> {
    fn map(data_in: Receiver<Data>,
           data_out: Sender<Output>);
}

pub struct Incrementer;
impl Mapper<i32, i32> for Incrementer {
    fn map(data_in: Receiver<i32>,
           data_out: Sender<i32>) {
        println!("Starting");
        loop {
            match data_in.recv() {
                Ok(x) => println!("Data received: {}", x),
                Err(_) => break, // Assume channel closed
            }
        }
    }
}
