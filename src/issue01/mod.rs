//! スタンダードなやつ

use std::fmt::Display;

/// fizzかbuzzかfizzbuzzか数値を表すenum
///
/// 3の倍数か5の倍数か3と5の公倍数か、それ以外を表します。
enum FizzBuzzNumber {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u32),
}
impl Display for FizzBuzzNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FizzBuzzNumber::Fizz => write!(f, "fizz!"),
            FizzBuzzNumber::Buzz => write!(f, "buzz!"),
            FizzBuzzNumber::FizzBuzz => write!(f, "fizzbuzz!"),
            FizzBuzzNumber::Number(n) => write!(f, "{}", n),
        }
    }
}
impl FizzBuzzNumber {
    fn new(n: u32) -> Self {
        match (n % 3, n % 5) {
            (0, 0) => Self::FizzBuzz,
            (0, _) => Self::Fizz,
            (_, 0) => Self::Buzz,
            _ => Self::Number(n),
        }
    }
}
