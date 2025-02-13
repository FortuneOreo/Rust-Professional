pub fn goldbach_conjecture() -> String {
    let mut data = 9;
    let mut res = Vec::new();

    while res.len() < 2 {
        if is_prime(data) {
            data += 2;
            continue;
        }

        let max_k = (((data - 2) / 2) as f64).sqrt().floor() as u64;
        let valid = (1..=max_k).any(|k| is_prime(data - 2 * k * k));
        if !valid {
            res.push(data);
        }

        data += 2;
    }

    format!("{},{}", res[0], res[1])
}

fn is_prime(val: u64) -> bool {
    if val <= 1 { return false; }
    if val <= 3 { return true; }
    if val % 2 == 0 { return false; }

    let max = (val as f64).sqrt() as u64;
    (3..=max).step_by(2).all(|i| val % i != 0)
}
