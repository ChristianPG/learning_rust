fn main() {
    print_labeled_measurement(5, '😬');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}