use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let t1 = thread::spawn(|| {
        for i in 0..20 {
            println!("\tI'm a new Thread! Count is {}", i);
            sleep(Duration::from_secs(1));
        }
    });

    for i in 0..10 {
        println!("I'm the main Thread! Count is {}", i);
        sleep(Duration::from_secs(1));
    }

    println!(">> Main Thread is done!");
    println!(">> It's waiting for the new Thread to finish.");
    t1.join().unwrap();
}
