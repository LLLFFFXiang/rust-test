// 从命令行读取一个整数 n（若读取失败或没有输入则默认 n = 5）。
// 打印从 1 到 n 的所有整数，每行一个。
// 若该整数可以被 3 整除，则在数字后面附加输出 "Fizz"；若可以被 5 整除，则附加输出 "Buzz"；若同时满足可以被 3 和 5 整除的情况，则输出 "FizzBuzz"。
use std::env;

pub fn fizzbuzz(n: u32) -> Vec<String> {
    (1..=n).map(|i| {
        match (i % 3, i % 5) {
            (0, 0) => format!("{}FizzBuzz", i),
            (0, _) => format!("{}Fizz", i),
            (_, 0) => format!("{}Buzz", i),
            _ => i.to_string(),
        }
    }).collect()
}

fn main() {
    let n = match env::args().nth(1) {  // 获取第一个命令行参数
        Some(arg) => arg.parse().unwrap_or_else(|_| {
            eprintln!("错误：参数必须是整数，默认使用 n=5");
            5
        }),
        None => 5,
    };

    for result in fizzbuzz(n) {
        println!("{}", result);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz_basic() {
        assert_eq!(fizzbuzz(1), vec!["1"]);
        assert_eq!(fizzbuzz(2), vec!["1", "2"]);
    }

    #[test]
    fn test_fizzbuzz_fizz() {
        assert_eq!(fizzbuzz(3), vec!["1", "2", "3Fizz"]);
    }

    #[test]
    fn test_fizzbuzz_buzz() {
        assert_eq!(fizzbuzz(5), vec!["1", "2", "3Fizz", "4", "5Buzz"]);
    }

    #[test]
    fn test_fizzbuzz_combined() {
        assert_eq!(
            fizzbuzz(15),
            vec![
                "1", "2", "3Fizz", "4", "5Buzz",
                "3Fizz", "7", "8", "3Fizz", "5Buzz",
                "11", "3Fizz", "13", "14", "15FizzBuzz"
            ]
        );
    }

    #[test]
    fn test_fizzbuzz_edge_cases() {
        assert_eq!(fizzbuzz(0), Vec::<String>::new());
    }
}