use std::collections::HashMap;

fn get_mode(set: &mut HashMap<i32, u32>) {
    println!("{:?}", set);
}

fn main() {
    // Given a list of integers, use a vector and return the median (when sorted, value in the
    // middle) and mode (value that occurs most often (use a hashmap))
    let mut set = HashMap::new();
    set.entry(50).or_insert(1);
    set.entry(999).or_insert(1);
    set.entry(192).or_insert(1);

    set.entry(50).and_modify(|e| *e += 1).or_insert(1);
    get_mode(&mut set);
}
