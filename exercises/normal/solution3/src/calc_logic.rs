pub fn new_birthday_probability(n: u32) -> f64 {
    if n > 365 {
    return 1 as f64;
    }

    let mut probability = 1 as f64;
    for i in 0..n {
        probability *= (365 - i) as f64 / 365.0;
    }

    1 as f64 - probability
}
