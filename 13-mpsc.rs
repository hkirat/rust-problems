use std::{
    sync::mpsc,
    thread::{self, spawn},
};

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let producer = tx.clone();
        spawn(move || {
            let mut sum: u64 = 0;
            for j in i * 10000000..(i + 1 * 10000000) - 1 {
                sum = sum + j;
            }
            producer.send(sum).unwrap();
        });
    }
    drop(tx);

    let mut final_sum: u64 = 0;
    for val in rx {
        println!("recv value from thread");
        final_sum = final_sum + val;
    }
    println!("{}", final_sum);
}

