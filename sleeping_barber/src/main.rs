use std::thread;

fn main() {
    println!("Begin!");
    thread::spawn(move || { barber() } );

    thread::sleep_ms(5000);
    println!("Done!");
}

fn barber() {
    println!("Barber arrives at work");
    loop {
       println!("Barber going for a nap");
       thread::sleep_ms(500);
       println!("Barber wakes up");
    }
}

