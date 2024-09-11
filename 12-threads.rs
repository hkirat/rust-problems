use std::thread;

fn main() {
    thread::spawn(|| {
        let mut c = 0;
        for i in 0..500000000 {
            for j in 0..500000000 {
                c = c + 1;
                c = c - 1;
            }
        }
    });

    thread::spawn(|| {
        let mut c = 0;
        for i in 0..500000000 {
            for j in 0..500000000 {
                c = c + 1;
                c = c - 1;
            }
        }
    });

    let mut c = 0;
    for i in 0..500000000 {
        for j in 0..500000000 {
            c = c + 1;
            c = c - 1;
            // println!("hi from spawned thread {}", i);
        }
    }
}

