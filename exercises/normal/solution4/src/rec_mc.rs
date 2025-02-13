pub fn dp_rec_mc(amount: u32) -> u32 {
    let cashes = [1, 2, 5, 10, 20, 30, 50, 100];

    if amount == 0 {
        return 0;
    }

    let mut obj = amount;
    let mut used_cash = Vec::new();

    while obj > 0 {
        for i in 0..8 as usize {
            if cashes[i as usize] <= obj {
                if i == 7 || cashes[i + 1] > obj {
                    used_cash.push(cashes[i]);
                    break;
                }
            }
        }

        obj -= used_cash.last().unwrap();
    }

    used_cash.len() as u32
}
