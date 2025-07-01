//! スタンダードなやつ

use std::fmt::Display;

/// [FizzBuzzNumber]もつ構造体
///
/// [FizzBuzzNumber]を隠蔽するために存在します。
pub struct FizzBuzz(FizzBuzzNumber);
impl FizzBuzz {
    pub fn new(n: u32) -> Self {
        if n == 0 {
            return Self(FizzBuzzNumber::Number(0));
        }
        match (n % 3, n % 5) {
            (0, 0) => Self(FizzBuzzNumber::FizzBuzz),
            (0, _) => Self(FizzBuzzNumber::Fizz),
            (_, 0) => Self(FizzBuzzNumber::Buzz),
            _ => Self(FizzBuzzNumber::Number(n)),
        }
    }
}
impl Display for FizzBuzz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

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

#[cfg(test)]
mod tests {
    use crate::issue01::FizzBuzz;

    #[test]
    fn t_fizz() {
        let number = 3;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "fizz!")
    }

    #[test]
    fn t_buzz() {
        let number = 5;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "buzz!")
    }

    #[test]
    fn t_fizzbuzz() {
        let number = 15;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "fizzbuzz!")
    }

    #[test]
    fn t_number_0() {
        let number = 0;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "0")
    }
    #[test]
    fn t_number_1() {
        let number = 1;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "1")
    }
    #[test]
    fn t_number_2() {
        let number = 2;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "2")
    }
    #[test]
    fn t_number_3() {
        let number = 3;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "fizz!")
    }
    #[test]
    fn t_number_4() {
        let number = 4;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "4")
    }
    #[test]
    fn t_number_5() {
        let number = 5;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "buzz!")
    }
    #[test]
    fn t_number_6() {
        let number = 6;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "fizz!")
    }
    #[test]
    fn t_number_7() {
        let number = 7;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "7")
    }
    #[test]
    fn t_number_8() {
        let number = 8;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "8")
    }
    #[test]
    fn t_number_9() {
        let number = 9;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "fizz!")
    }
    #[test]
    fn t_number_10() {
        let number = 10;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "buzz!")
    }
    #[test]
    fn t_number_11() {
        let number = 11;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "11")
    }
    #[test]
    fn t_number_12() {
        let number = 12;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "fizz!")
    }
    #[test]
    fn t_number_13() {
        let number = 13;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "13")
    }
    #[test]
    fn t_number_14() {
        let number = 14;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "14")
    }
    #[test]
    fn t_number_15() {
        let number = 15;
        let result = FizzBuzz::new(number);

        assert_eq!(result.to_string(), "fizzbuzz!")
    }
}
