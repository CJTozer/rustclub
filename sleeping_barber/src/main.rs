use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    println!("Begin!");
    let mut queue = Arc::new(Mutex::new(Vec::new()));
    let queue_copy = queue.clone();
    thread::spawn(move || { barber(queue_copy) } );
    thread::spawn(move || { customers(queue.clone()) } );

    thread::sleep_ms(2000);
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
                thread::sleep_ms(500);
                println!("Barber wakes up");
            },
            Some(cust) => {
                println!("Cutting customer {}'s hair", cust);
                thread::sleep_ms(100);
                println!("Done cutting");
            }
        }
    }
}

fn customers(queue_arc: Arc<Mutex<Vec<u32>>>) {
    println!("Customers start heading to the barbers");
    let mut cust_no = 1;
    loop {
        let mut queue = queue_arc.clone();
        let this_cust = cust_no;
        thread::spawn(move || {
            println!("Customer {} enters shop", this_cust);
            let mut q = queue.lock().unwrap();

        });

        thread::sleep_ms(100);
    }
}
