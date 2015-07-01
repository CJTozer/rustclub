use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    println!("Begin!");
    let mut shop = Arc::new(Mutex::new(BarberShop{
                                           free_waiting_seats: 5,
                                           customer_queue: Vec::new() }));
    thread::spawn(move || { barber(shop.clone()) } );

    thread::sleep_ms(2000);
    println!("Done!");
}

fn barber(shop_arc: Arc<Mutex<BarberShop>>) {
    println!("Barber arrives at work");
    loop {
        println!("Barber checks queue");
        let mut shop = shop_arc.lock().unwrap();
        match shop.customer_queue.pop() {
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
        drop(shop);
    }
}

struct BarberShop {
    free_waiting_seats: u32,
    customer_queue: Vec<u32>,
}
