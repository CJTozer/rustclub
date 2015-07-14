extern crate rand;

use std::thread;
use std::sync::{Arc, Mutex, Condvar};
use rand::Rng;

const NUM_SEATS: usize = 5;

fn main() {
    println!("Begin!");
    let pair = Arc::new((Mutex::new(Vec::new()), Condvar::new()));
    let pair_copy = pair.clone();
    thread::spawn(move || { barber(pair_copy) } );
    thread::spawn(move || { customers(pair.clone()) } );

    thread::sleep_ms(10000);
    println!("Done!");
}

fn barber(pair_arc: Arc<(Mutex<Vec<u32>>, Condvar)>) {
    let (ref mutex, ref condvar) = *(pair_arc.clone());
    println!("Barber arrives at work");
    loop {
        println!("Barber checks queue");
        let mut queue = mutex.lock().unwrap();
        let cust = queue.pop();
        drop(queue);
        match cust {
            None => {
                println!("No customers waiting");
                println!("Barber going for a nap");
                condvar.wait(mutex.lock().unwrap()).ok();
                println!("Barber woken up");
            },
            Some(cust) => {
                println!("Cutting customer {}'s hair", cust);
                thread::sleep_ms(500);
                println!("Done cutting");
            }
        }
    }
}

fn customers(pair_arc: Arc<(Mutex<Vec<u32>>, Condvar)>) {
    println!("Customers start heading to the barbers");
    let mut rng = rand::thread_rng();
    let mut cust_no = 1;
    loop {
        let this_cust = cust_no;
        let this_arc = pair_arc.clone();
        thread::spawn(move || {
            println!("Customer {} enters shop", this_cust);
            let (ref mutex, ref condvar) = *this_arc;
            let mut queue = mutex.lock().unwrap();
            if queue.len() < NUM_SEATS {
                println!("Space for one more waiting");
                queue.push(cust_no);
                condvar.notify_one();
            } else {
                println!("No space remaining");
            }
            drop(queue);
        });

        thread::sleep_ms(rng.gen::<u32>() % 500);
        cust_no += 1;
    }
}
