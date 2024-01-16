use std::collections::HashMap;

fn main() {
    let mut initial_values: Vec<i32> = vec![1, 5, 7, 9, 9, 8, 5];
    println!("Initial vector: {:?}", initial_values);

    // Calculate the Median
    initial_values.sort();
    println!("Sorted vector: {:?}", initial_values);

    let mut _median = 0.0;
    let vector_length = initial_values.len();

    if vector_length % 2 == 0 {
        _median = (initial_values[vector_length / 2] + initial_values[(vector_length / 2) - 1])
            as f64
            / 2.0;
    } else {
        _median = initial_values[(vector_length - 1) / 2] as f64;
    }
    println!("Median: {_median}");

    // Calculate the Mode
    let mut mode_map = HashMap::<i32, i32>::new();

    for i in initial_values {
        let count = mode_map.entry(i).or_insert(0);
        *count += 1;
    }
    println!("Map of numbers with repetition count: {:?}", mode_map);

    let mut _mode = String::new();
    let mut max_repetition = 0;
    for (key, value) in mode_map {
        if value > max_repetition {
            max_repetition = value;
            _mode = key.to_string();
        } else if value == max_repetition {
            let key_as_string = key.to_string();
            _mode = format!("{_mode}, {key_as_string}");
        }
    }
    println!("Mode: {_mode}");
}
