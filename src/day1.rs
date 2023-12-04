use aoc_runner_derive::{aoc, aoc_generator};

use fancy_regex::Regex;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> String {
    String::from(input.trim())
}

#[aoc(day1, part1)]
pub fn part1(input: &String) -> i32 {
    let lines = input.split("\n");

    let mut sum = 0;

    for line in lines {
        let mut first: Option<i32> = None;
        let mut last: Option<i32> = None;

        for char in line.chars() {
            match char.to_string().parse::<i32>() {
                Ok(num) => {
                    if first.is_none() {
                        first = Some(num);
                    }

                    last = Some(num);
                }
                Err(_) => (),
            }
        }

        sum += first.unwrap() * 10 + last.unwrap()
    }

    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &String) -> i32 {
    let lines = input.split("\n");

    let mut sum = 0;

    let re = Regex::new(
        r"(?P<word_number>one|two|three|four|five|six|seven|eight|nine|ten)|(?P<number>\d+)",
    )
    .unwrap();

    for line in lines {
        let mut first: Option<i32> = None;
        let mut last: Option<i32> = None;

        let caps = re.captures(line).unwrap();

        for cap in caps {
            let mut number: Option<i32> = None;
            if let Some(word_number) = cap.name("word_number") {
                number = match word_number.as_str() {
                    "one" => Some(1),
                    "two" => Some(2),
                    "three" => Some(3),
                    "four" => Some(4),
                    "five" => Some(5),
                    "six" => Some(6),
                    "seven" => Some(7),
                    "eight" => Some(8),
                    "nine" => Some(9),
                    _ => None,
                }
            }

            if let Some(num) = cap.name("number") {
                number = Some(num.as_str().parse::<i32>().unwrap());
            }

            println!("{:?} {:?}", number, cap);

            if let Some(num) = number {
                if first.is_none() {
                    first = Some(num);
                }

                last = Some(num);
            }
        }

        println!("{:?}, {:?}", first, last);

        sum += first.unwrap() * 10 + last.unwrap();

        return sum;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_1: &'static str = indoc! {"
		1abc2
		pqr3stu8vwx
		a1b2c3d4e5f
		treb7uchet
    "};

    static EXAMPLE_2: &'static str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    static REDDIT_EXAMPLE_1: &'static str = indoc! {"
        eighthree
        sevenine
    "};

    #[test]
    fn day1_part1_example1() {
        assert_eq!(part1(&input_generator(EXAMPLE_1)), 142);
    }

    #[test]
    fn day1_part2_example1() {
        assert_eq!(part2(&input_generator(EXAMPLE_1)), 142);
    }

    #[test]
    fn day1_part2_example2() {
        assert_eq!(part2(&input_generator(EXAMPLE_2)), 281);
    }

    #[test]
    fn day1_part2_reddit_example1() {
        assert_eq!(part2(&input_generator(REDDIT_EXAMPLE_1)), 162);
    }
}
