pub fn recite(start_bottles: u32, take_down: u32) -> String {
    fn num_word(n: u32, capitalize: bool) -> &'static str {
        let word = match n {
            0 => "no",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            _ => "many",
        };
        if capitalize {
            match word {
                "no" => "No",
                "one" => "One",
                "two" => "Two",
                "three" => "Three",
                "four" => "Four",
                "five" => "Five",
                "six" => "Six",
                "seven" => "Seven",
                "eight" => "Eight",
                "nine" => "Nine",
                "ten" => "Ten",
                _ => word,
            }
        } else {
            word
        }
    }

    fn bottle_word(n: u32) -> &'static str {
        if n == 1 { "bottle" } else { "bottles" }
    }

    let mut verses = Vec::new();

    for n in (start_bottles - take_down + 1..=start_bottles).rev() {
        let next = n - 1;

        let verse = format!(
            "{num} green {bottles} hanging on the wall,\n\
             {num} green {bottles} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {next_num} green {next_bottles} hanging on the wall.",
            num = num_word(n, true),
            bottles = bottle_word(n),
            next_num = num_word(next, false),
            next_bottles = bottle_word(next)
        );

        verses.push(verse);
    }

    verses.join("\n\n")
}
