fn primes(max: u32) -> Vec<u32> {
    let mut primes = vec![2];
    for n in 3..max {
        if primes.iter().all(|p| n % p != 0) {primes.push(n);}
    }
    primes
}

fn main() {
    println!("{:?}", primes(1000));
}
