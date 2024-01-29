fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn calculate_primes(range: (u32, u32)) -> Vec<u32> {
    let mut primes = Vec::new();
    for num in range.0..range.1 {
        if is_prime(num) {
            primes.push(num);
        }
    }
    primes
}