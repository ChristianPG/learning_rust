fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    const ARRAY_OF_STRINGS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let string = "asdf";
    let mut integer = 5;
    println!("The value of integer is: {integer}");
    integer = 6;
    println!("The value of integer is: {integer}");
}
