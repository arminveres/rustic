fn get_median(vector: &mut Vec<i32>) {
    vector.sort();
    let middle = vector.len() / 2;
    let median = vector.get(middle);
    let median_slice = vector[middle];
    println!("{:?}", median);
    println!("{median_slice}");
}

fn main() {
    // Given a list of integers, use a vector and return the median (when sorted, value in the
    // middle) and mode (value that occurs most often (use a hashmap))
    let mut test_vec = vec![10, 40, 2000, 999, 9999];
    get_median(&mut test_vec);
}
