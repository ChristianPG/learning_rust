fn main() {
    const ARRAY_OF_LINES: [&str; 13] = [
        "On the twelfth day of Christmas, my true love sent to me",
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves, and",
        "A partridge in a pear tree",
    ];

    let mut dynamic_verse_lines: [&str; 13] = Default::default();
    for verse_index in 1..=12 {
        if verse_index == 1 {
            dynamic_verse_lines[0] = ARRAY_OF_LINES[0];
            dynamic_verse_lines[12] = ARRAY_OF_LINES[12];
        } else {
            dynamic_verse_lines[dynamic_verse_lines.len() - verse_index] =
                ARRAY_OF_LINES[dynamic_verse_lines.len() - verse_index]
        }

        println!("\n[Verse {}]", verse_index);
        for line in dynamic_verse_lines {
            if line != "" {
                println!("{}", line)
            }
        }
    }
}
