static INPUT: &'static str = "1113222113";

fn look_and_say(input_string: &str) -> String {
    let mut s = String::new();
    let mut prev_num = 0;
    let mut prev_count = 0;
    let str_len = input_string.chars().count();
    for (i, c) in input_string.chars().enumerate() {
        let d = c.to_digit(10).unwrap();
        if prev_num == 0 {
            prev_num = d;
        }
        if prev_num == d {
            prev_count += 1;
        }
        if prev_num != d {
            s.push_str(&prev_count.to_string());
            s.push_str(&prev_num.to_string());
            prev_num = d;
            prev_count = 1;
        }

        if i == (str_len - 1) {
            s.push_str(&prev_count.to_string());
            s.push_str(&prev_num.to_string());
        }

        // println!("{}", s);
    }
    return s.to_string();
}

#[test]
fn test() {
    assert_eq!("11", look_and_say("1"));
    assert_eq!("21", look_and_say("11"));
    assert_eq!("1211", look_and_say("21"));
    assert_eq!("312211", look_and_say("111221"));
}

fn part1() {
    evaluate(40);
}

fn part2() {
    evaluate(50);
}

fn evaluate(N: usize) {
    let mut input = INPUT.to_string();
    for _ in 0..N {
        input = look_and_say(&input);
    }
    println!("{}", input.chars().count());
}

fn main() {
    part2();
}
