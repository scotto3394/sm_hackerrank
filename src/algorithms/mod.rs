//! This module details progress through the `Algorithms`
//! track of HackerRank.
//!
//! This file focuses on the `Warmup` subtrack.
pub mod implementation;

/// Builds a staircase from #.
pub fn staircase(n: u8) -> String {
    let mut buffer = String::with_capacity((n * (n + 1)) as usize);
    for level in 1..(n + 1) {
        for _ in 0..(n - level) {
            buffer.push(' ');
        }
        for _ in 0..level {
            buffer.push('#');
        }
        buffer.push('\n');
    }
    buffer
}

/// Finds the minimum and maximum sum of a 4 subset of 5.
pub fn minimax_sum(n: Vec<u64>) -> (u64, u64) {
    assert_eq!(5, n.len());
    let mut subset = n[1] + n[2] + n[3] + n[4];
    let mut min = subset;
    let mut max = subset;
    for index in 1..5 {
        subset -= n[index];
        subset += n[index - 1];
        if subset < min {
            min = subset;
        }
        if subset > max {
            max = subset;
        }
    }
    (min, max)
}

/// Count the number of maximum values in an array.
pub fn birthday_cake(n: Vec<u32>) -> u32 {
    let mut max = 0;
    let mut num_max = 0;
    for num in &n {
        if *num == max {
            num_max += 1;
        } else if *num > max {
            max = *num;
            num_max = 1;
        }
    }
    num_max
}

/// Convert from AM/PM format to military time.
pub fn time_conversion(n: String) -> String {
    let parts: Vec<&str> = n.split(':').collect();
    let mut hour: u8 = parts[0].parse().unwrap();
    let format = &parts[2][2..4];
    hour = hour % 12;
    if format == "PM" {
        hour += 12;
    }

    format!("{:02}:{}:{}", hour, parts[1], &parts[2][0..2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stairs() {
        let n = 6;
        let solution = String::from("     #\n    ##\n   ###\n  ####\n #####\n######\n");
        assert_eq!(solution, staircase(n));
    }

    #[test]
    fn test_minimax() {
        let n = vec![1, 2, 3, 4, 5];
        let solution = (10, 14);
        assert_eq!(solution, minimax_sum(n));
    }

    #[test]
    fn test_birthday_cake() {
        let n = vec![3, 1, 2, 3];
        let solution = 2;
        assert_eq!(solution, birthday_cake(n));
    }

    #[test]
    fn test_time_conversion() {
        let n = String::from("07:05:45PM");
        let solution = String::from("19:05:45");

        assert_eq!(solution, time_conversion(n));
    }
}
