extern crate rand;

use std::thread;
use std::sync::{Arc, Mutex};
use rand::Rng;

const NUM_SEATS: usize = 5;

fn main() {
    println!("Begin!");
    let mut queue = Arc::new(Mutex::new(Vec::new()));
    let queue_copy = queue.clone();
    thread::spawn(move || { barber(queue_copy) } );
    thread::spawn(move || { customers(queue.clone()) } );

    thread::sleep_ms(10000);
    println!("Done!");
}

fn barber(queue_arc: Arc<Mutex<Vec<u32>>>) {
    println!("Barber arrives at work");
    loop {
        println!("Barber checks queue");
        let mut queue = queue_arc.lock().unwrap();
        let cust = queue.pop();
        drop(queue);
        match cust {
            None => {
                println!("No customers waiting");
                println!("Barber going for a nap");
                thread::sleep_ms(1000);
                println!("Barber wakes up");
            },
            Some(cust) => {
                println!("Cutting customer {}'s hair", cust);
                thread::sleep_ms(500);
                println!("Done cutting");
            }
        }
    }
}

fn customers(queue_arc: Arc<Mutex<Vec<u32>>>) {
    println!("Customers start heading to the barbers");
    let mut rng = rand::thread_rng();
    let mut cust_no = 1;
    loop {
        let mut queue = queue_arc.clone();
        let this_cust = cust_no;
        thread::spawn(move || {
            println!("Customer {} enters shop", this_cust);
            let mut q = queue.lock().unwrap();
            if (q.len() < NUM_SEATS) {
                println!("Space for one more waiting");
                q.push(cust_no);
            } else {
                println!("No space remaining");
            }
            drop(q);
        });

        thread::sleep_ms(rng.gen::<u32>() % 1000);
        cust_no += 1;
    }
}
