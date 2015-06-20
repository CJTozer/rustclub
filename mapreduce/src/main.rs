use std::sync::mpsc::channel;
use std::thread;

mod mapper;
use mapper::Mapper;

fn main() {
    let (mapper_input_tx, mapper_input_rx) = channel();
    let (mapper_output_tx, mapper_output_rx) = channel();

    thread::spawn(move || {
        mapper::Incrementer::map(mapper_input_rx, mapper_output_tx);
    });

    for ii in 0..10 {
        println!("Sending {}", ii);
        mapper_input_tx.send(ii).unwrap();
    }

    // Close the input channel
    drop(mapper_input_tx);

    thread::sleep_ms(1000);
    println!("Done");
}
