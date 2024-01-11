fn main() {
    let mut initial_values: Vec<i32> = vec![1, 5, 7, 9, 8];
    println!("Initial vector: {:?}", initial_values);

    initial_values.sort();
    println!("Sorted vector: {:?}", initial_values);

    let mut _median = 0.0;
    let vector_length = initial_values.len();

    if vector_length % 2 == 0 {
        _median = (initial_values[vector_length / 2] + initial_values[(vector_length / 2) - 1]) as f64 / 2.0;
    } else {
        _median = initial_values[(vector_length - 1) / 2] as f64;
    }
    println!("Median: {_median}")
}
