use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    println!("Begin!");
    let mut shop = Arc::new(Mutex::new(BarberShop{
                                           free_waiting_seats: 5,
                                           customer_queue: Vec::new() }));
    let shop_copy = shop.clone();
    thread::spawn(move || { barber(shop_copy) } );
    thread::spawn(move || { customers(shop.clone()) } );

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
                drop(shop);
                println!("Barber going for a nap");
                thread::sleep_ms(500);
                println!("Barber wakes up");
            },
            Some(cust) => {
                println!("Cutting customer {}'s hair", cust);
                shop.free_waiting_seats += 1;
                drop(shop);
                thread::sleep_ms(100);
                println!("Done cutting");
            }
        }
    }
}

fn customers(shop_arc: Arc<Mutex<BarberShop>>) {
    println!("Customers start heading to the barbers");
    let mut cust_no = 1;
    loop {
        let mut shop_copy = shop_arc.clone();
        let this_cust = cust_no;
        thread::spawn(move || {
            println!("Customer {} enters shop", this_cust);
            let mut shop = shop_copy.lock().unwrap();
            println!("Spare seats = {}", shop.free_waiting_seats);

        });

        thread::sleep_ms(100);
    }
}

struct BarberShop {
    free_waiting_seats: u32,
    customer_queue: Vec<u32>,
}
