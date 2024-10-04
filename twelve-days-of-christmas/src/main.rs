fn main() {
    const TWELVE_DAYS: &[&str] = &[
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    const DAYS: &[&str] = &[
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for verse in 1..12 + 1 {
        println!("On the {} day of Christmas,", DAYS[verse - 1]);
        println!("my true love sent to me");

        for day in (1..verse + 1).rev() {
            let gift = TWELVE_DAYS[day - 1];
            let first_char =
                std::ascii::AsciiExt::to_ascii_uppercase(&gift.chars().next().unwrap());
            if verse > 1 && day == 1 {
                println!("And {}.", &gift);
            } else if day == 1 {
                println!("{}{}.", first_char, &gift[1..]);
            } else if verse == 12 && day == 12 {
                println!("{}{}!", first_char, &gift[1..]);
            } else {
                println!("{}{},", first_char, &gift[1..]);
            }
        }
        println!("");
    }
}
