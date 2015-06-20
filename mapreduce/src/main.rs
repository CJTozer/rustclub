use std::sync::mpsc::channel;
use std::thread;

mod mapper;
use mapper::Mapper;

fn main() {
    // Channel for getting mapped output
    let (mapper_output_tx, mapper_output_rx) = channel();

    for ii in 0..4 {
        // Channel for passing data to this mapper
        let (mapper_input_tx, mapper_input_rx) = channel();
        // Each mapper gets a copy of the output TX channel
        let this_mapper_output_tx = mapper_output_tx.clone();
        println!("Creating mapper {}", ii);
        thread::spawn(move || {
            mapper::Summer::map(mapper_input_rx,
                                this_mapper_output_tx);
        });

        for jj in 0..10 {
            println!("Sending {}", jj);
            mapper_input_tx.send(jj).unwrap();
        }

        // Close the input channel
        drop(mapper_input_tx);
    }

    // Drop the copy of the output TX channel that we still own
    drop(mapper_output_tx);

    // Receive the output
    loop {
        match mapper_output_rx.recv() {
            Ok(x) => println!("Got result: {}", x),
            Err(_) => break, // Assume channel closed
        }
    }

    println!("Done");
}
