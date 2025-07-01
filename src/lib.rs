//! アルゴリズムの勉強をしたくなったわけですよ。
//!
//! ## 本プログラムの解説について
//! [https://tam1192.github.io/algorithm_rs/](https://tam1192.github.io/algorithm_rs/)です。

pub mod issue01;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
