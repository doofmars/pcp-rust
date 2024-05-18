use std::thread;
use std::sync::mpsc::channel;

fn send_receive() {
    // create channel
    let (sender, receiver) = channel();

    // thread to send data
    let sending_thread = thread::spawn(move || {
        sender.send("important data")
            .expect("Could not send data")
    });

    // thread to receive data
    let receiving_thread = thread::spawn(move || {
        let data = receiver.recv()
            .expect("Did not receive data");

        println!("Received: {}", data)
    });

    // cleanup
    sending_thread.join().expect("Send has panicked");
    receiving_thread.join().expect("Receive has panicked");
}

struct Data {
    field: String,
}

fn channel_data_ownership() {
    // create channel for Data
    let (sender, receiver) = channel::<Data>();

    // the move keyword allows the thread to take ownership of "sender"
    let thread1 = thread::spawn(move || {
        // "data" is owned by thread 1 here
        let data = Data {
            field: String::from("my data"),
        };

        // data is moved to thread 2
        sender.send(data)
            .expect("Send failed");

        // The following line will not compile because data was moved
        // println!("Thread 1 has: {}", data.field)
    });

    // receive the data and add output it
    let thread2 = thread::spawn(move || {
        // receive the data and add output it
        let data= receiver.recv()
            .expect("Receive failed");

        println!("Thread 2 has {}", data.field)
    });

    // finish
    thread1.join().expect("Thread 1 panicked");
    thread2.join().expect("Thread 2 panicked");
}

fn main () {
    println!("Example: Send & Receive:");
    send_receive();
    println!("----");

    println!("Example: Data Ownership");
    channel_data_ownership();
    println!("----");
}
