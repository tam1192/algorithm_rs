# スタンダードなやつ
とりあえずこのリポジトリの運用方法を定めるために簡単なやつ出しておきます。

## 数式テスト

$$ 速度[m/s] = \frac {距離[m]} {時間[s]} $$
$$ 距離[m] = {速度[m/s]} \times {時間[s]} $$

```
$$ 速度[m/s] = \frac {距離[m]} {時間[s]} $$
$$ 距離[m] = {速度[m/s]} \times {時間[s]} $$
```

数式も用いて説明したい。

## fizzbuzz
整数Nが与えれます。 ($ 0 < N < 15 $)
Nが3の倍数の時は`fizz`と、
Nが5の倍数の時は`buzz`と、
Nが3と5の公倍数の時は`fizzbuzz`と、
それ以外の時はNを出力してくださいっていうあれ。

## 実装方法
とりあえずあまりを使って作ります。

数値nを3と5でそれぞれあまりを出し、両方とも0ならfizzbuzz、3側が0ならfizz、5側が0ならbuzzです。
rustではmatchでよしなに処理が可能だったりします。
```rust,editable
fn main() {
    // change here!
    let n = 10;
    match (n % 3, n % 5) {
        (0, 0) => println!("fizzbuzz"),
        (0, _) => println!("fizz"),
        (_, 0) => println!("buzz"),
        _ => println!("{}", n),
    }
}
```

## 実装結果
[ソースコードのドキュメントはこちら](rsDoc/algorithm_rs/issue01/)
enumとstructを使ってみました。　カッコつけたかっただけです。　はい。