pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    if threshold < 1 {
        return 0;
    }

    if threshold < 2 {
        return 2;
    }

    let mut list: Vec<u32> = vec![1];
    let mut fibn_2 = 0;
    let mut fibn_1 = 1;
    let mut fibn;

    loop {
        fibn = fibn_1 + fibn_2;
        if fibn >= threshold {
            break;
        }
        fibn_2 = fibn_1;
        fibn_1 = fibn;
        if fibn % 2 == 1 {
            list.push(fibn);
        }
    }

    list.iter().sum()
}
