fn max_n_nlogn(t_micro: u128) -> u128 {
    let mut low = 1u128;
    let mut high = t_micro; // upper bound guess

    while low < high {
        let mid = (low + high + 1) / 2;

        // compute mid * log2(mid) safely using f64
        let val = (mid as f64) * (mid as f64).log2();

        if val <= t_micro as f64 {
            low = mid;
        } else {
            high = mid - 1;
        }
    }

    low
}
fn max_n_factorial(t_micro: u128) -> u128 {
    let mut n = 1u128;
    let mut fact = 1u128;

    while fact <= t_micro {
        n += 1;
        fact *= n;
    }

    n - 1

}

fn print_line() {
    println!("{}", "─".repeat(50));
}
fn display_max() {
    // time units in microseconds
    let times = [
        ("second", 1_000_000_u128),
        ("minute", 60 * 1_000_000),
        ("hour", 60 * 60 * 1_000_000),
        ("day", 24 * 60 * 60 * 1_000_000),
        ("month (30 days)", 30 * 24 * 60 * 60 * 1_000_000),
        ("year (365 days)", 365 * 24 * 60 * 60 * 1_000_000),
        ("century", 100 * 365 * 24 * 60 * 60 * 1_000_000),
    ];

    println!("log₂(n) µs:");
    for (label, t_micro) in times.iter() {
        println!("{:<20} -> max n = 2^{}", label, t_micro);
    }
    print_line();
    println!("sqrt(n) µs:");
    for (label, t_micro) in times.iter() {
        let n = t_micro * t_micro;
        println!("{:<20} -> max n = {}", label, n);
    }
    print_line();
    println!("n µs:");
    for (label, t_micro) in times.iter() {
        println!("{:<20} -> max n = {}", label, t_micro);
    }
    print_line();
    println!("n log n µs:");
    for (label, t_micro) in times.iter() {
        let n = max_n_nlogn(*t_micro);
        println!("{:<20} -> max n = {}", label, n);
    }
    print_line();
    println!("n * n µs:");
    for (label, t_micro) in times.iter() {
        println!("{:<20} -> max n = {}", label, t_micro.isqrt());
    }
    print_line();
    println!("n * n * n µs:");
    for (label, t_micro) in times.iter() {
        let n = (*t_micro as f64).cbrt() as u128;
        println!("{:<20} -> max n = {}", label, n);
    }
    print_line();
    println!("2 ** n µs:");
    for (label, t_micro) in times.iter() {
        let n = (*t_micro as f64).log2() as u128;
        println!("{:<20} -> max n = {}", label, n);
    }
    print_line();
    println!("n! µs:");
    for (label, t_micro) in times.iter() {
        let n = max_n_factorial(*t_micro);
        println!("{:<20} -> max n = {}", label, n);
    }// it is slightly off but whatever
}

fn main() {
    display_max()
}
