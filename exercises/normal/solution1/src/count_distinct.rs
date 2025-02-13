use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let mut list: HashSet<String> = HashSet::new();
    for item in input_str.split(',') {
        list.insert(item.to_string());
    }

    list.len()
}
