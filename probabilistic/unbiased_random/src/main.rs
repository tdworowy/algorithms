use rand::RngExt;

fn biased_random(p: f64) -> u8 {
    let mut rng = rand::rng();
    if rng.random::<f64>() < p { 1 } else { 0 }
}
fn unbiased_random() -> u8 {
    let a = biased_random(0.7);
    let b = biased_random(0.7);
    if a == 0 && b == 1 {
        return 1;
    }
    if a == 1 && b == 0 {
        return 0;
    }
    unbiased_random()
}

fn unbiased_random_different_bias() -> u8 {
    let a1 = biased_random(0.2);
    let a2 = biased_random(0.2);
    let b1 = biased_random(0.8);
    let b2 = biased_random(0.8);

    let mut a: u8 = 0;
    let mut b: u8 = 0;

    if a1 == 0 && a2 == 1 {
        a = 1;
    } else if a1 == 1 && a2 == 0 {
        a = 0;
    } else {
        unbiased_random();
    };

    if b1 == 0 && b2 == 1 {
        b = 1;
    } else if b1 == 1 && b2 == 0 {
        b = 0;
    } else {
        unbiased_random();
    };

    if a == 0 && b == 1 {
        return 1;
    }
    if a == 1 && b == 0 {
        return 0;
    }
    unbiased_random()
}

#[test]
fn test_unbiased_random_is_close_to_fair() {
    let trials = 100_000;
    let mut ones = 0u32;
    for _ in 0..trials {
        ones += unbiased_random() as u32;
    }
    let p = ones as f64 / trials as f64;

    println!("Estimated p(1) = {}", p);
    // Allow small statistical error
    let epsilon = 0.01;
    assert!(
        (p - 0.5).abs() < epsilon,
        "Distribution too biased: p = {}",
        p
    );
}

#[test]
fn test_unbiased_random_is_close_to_fair_different_bias() {
    let trials = 100_000;
    let mut ones = 0u32;
    for _ in 0..trials {
        ones += unbiased_random_different_bias() as u32;
    }
    let p = ones as f64 / trials as f64;

    println!("Estimated p(1) = {}", p);
    // Allow small statistical error
    let epsilon = 0.01;
    assert!(
        (p - 0.5).abs() < epsilon,
        "Distribution too biased: p = {}",
        p
    );
}

#[test]
fn test_biased_random_is_biased() {
    let trials = 100_000;
    let mut ones = 0u32;
    for _ in 0..trials {
        ones += biased_random(0.7) as u32;
    }
    let p = ones as f64 / trials as f64;

    println!("Estimated p(1) = {}", p);
    // Allow small statistical error
    let epsilon = 0.01;
    assert!(
        (p - 0.7).abs() < epsilon,
        "Distribution too biased: p = {}",
        p
    );
}

fn main() {
    let result = unbiased_random();
    println!("{:?}", result);
}
