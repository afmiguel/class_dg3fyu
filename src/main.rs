use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut handles = vec![];
    for thread_number in 0..3{
        let t1 = thread::spawn(move || {
            for i in 0..20 {
                println!("\tI'm a new Thread {thread_number}! Count is {}", i);
                sleep(Duration::from_millis(700));
            }
        });
        handles.push(t1);
    }

    for i in 0..10 {
        println!("I'm the main Thread! Count is {}", i);
        sleep(Duration::from_secs(1));
    }

    println!(">> Main Thread is done!");
    println!(">> It's waiting for the new Thread to finish.");
    for h in handles{
        h.join().unwrap();
    }
}
