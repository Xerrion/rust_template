use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use rust_template::calculate_primes;


fn main() {
    let start = Instant::now();
    let range = 10000000;
    let num_threads = 4;
    let segment_size = range / num_threads;
    let (tx, rx) = mpsc::channel();

    for i in 0..num_threads {
        let start_range = i * segment_size + 2;
        let end_range = if i == num_threads - 1 {
            range
        } else {
            (i + 1) * segment_size + 2
        };
        let tx_clone = tx.clone();

        thread::spawn(move || {
            let primes = calculate_primes((start_range, end_range));
            tx_clone.send(primes).unwrap();
        });
    }

    let mut all_primes = Vec::new();
    for _ in 0..num_threads {
        let mut primes = rx.recv().unwrap();
        all_primes.append(&mut primes);
    }

    let total_time = format!("Total time: {:?}", start.elapsed());
    let found_primes_str = format!("Found {} prime numbers.", all_primes.len());

    println!("{}", found_primes_str);
    println!("{}", total_time);
}
