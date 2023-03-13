pub(crate) fn div(x: i32, y: i32) -> i32 {
    x / y
}

// #[test]アトリビュートをつけると cargo testで実行できる
#[test]
fn div_test() {
    assert_eq!(div(10, 3), 3);
}

#[test]
#[should_panic]
fn div_panic_test() {
    div(2, 0);
}

// テスト時以外はコンパイルされないように別のモジュールとする書き方
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div_test() {
        assert_eq!(div(10, 3), 3);
    }
}
