use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel();
    for ii in 0..10 {
        let tx = tx.clone();
        thread::spawn(move|| {
            tx.send(ii).unwrap();
        });
    }

    for _ in 0..10 {
        let jj = rx.recv().unwrap();
        println!("Received: {}", jj);
    }
}

