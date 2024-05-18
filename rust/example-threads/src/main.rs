use std::{panic, thread};
use std::fmt::Error;
use std::thread::JoinHandle;
use std::time::Duration;

fn example_thread() {
    println!("Start main thread");

    // start a thread
    let thread = thread::spawn(|| {
        println!("  Start other thread");

        // sleep for 10 milliseconds
        thread::sleep(Duration::from_millis(10));

        println!("  End other thread");
    });

    println!("Other thread started");

    // wait for the other thread to finish
    thread.join().expect("Other thread panicked");

    println!("Main thread")
}

fn example_with_result() {
    let successful_thread = thread::spawn(|| {
        // return a result
        "Success"
    });

    let failed_thread = thread::spawn(|| {
        // panic to return an Error
        panic!("Fail")
    });

    // wait for successful_thread and print the result
    println!("Successful result: {}", successful_thread.join().expect("successful thread panicked"));

    // check if the failed_thread has panicked
    if (failed_thread.join().is_err()) {
        println!("Failed thread panicked");
    }
}

fn main() {
    println!("Thread example:");
    example_thread();
    println!("\nExample with result:");

    example_with_result();
}
