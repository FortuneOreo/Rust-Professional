pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let mut iter = num_str.split(&['(', ')'][..]);
    let src_num = iter.next().expect("invalid num_str, need value str");
    let from_base = iter.next().expect("invalid num_str, need base str").parse().unwrap();
    
    let src_val = src_num.chars().fold(0, |acc, c| {
        let val = c.to_digit(from_base).unwrap();
        acc * from_base + val
    });

    if src_val == 0 {
        return "0".to_string();
    }

    let mut result = Vec::new();
    let digits = [
        '0','1','2','3','4','5','6','7','8','9',
        'a','b','c','d','e','f'
    ];
    let mut num = src_val;

    while num > 0 {
        let rem = (num % to_base) as usize;
        result.push(digits[rem]);
        num /= to_base;
    }
 
    result.iter().rev().collect()
}
