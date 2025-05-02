use std::collections::HashMap;

pub fn print_top<T: PartialOrd + Copy + std::fmt::Display>(
    map: &HashMap<usize, T>,
    top_n: usize,
) {
    let mut items: Vec<_> = map.iter().collect();
    items.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    for (i, (k, v)) in items.iter().take(top_n).enumerate() {
        println!("{:>2}. Node {:>4} → {}", i + 1, k, v);
    }
}
