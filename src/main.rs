fn gen_primes(n: usize) -> Vec<usize> {
    let mut p = 2;
    let nums: Vec<usize> = (2..n).collect();
    let mut primes: Vec<bool> = vec![true; n];
    let mut pi;
    loop {
        // Cross off impossible primes
        for num in nums.iter() {
            if num % p == 0 && *num != p {
                primes[*num] = false;
            }
        }

        // Find next prime
        pi = p + 1;
        while pi < primes.len() {
            if primes[pi] == true {
                p = pi;
                break;
            }
            pi += 1;
        }
        if pi >= (primes.len() - 2) {
            break;
        }
    }

    primes
        .iter()
        .enumerate()
        .filter(|(_, is_prime)| **is_prime)
        .map(|(i, _)| i)
        .skip(2)
        .collect()
}

fn main() {
    println!("{:?}",gen_primes(100));
}
