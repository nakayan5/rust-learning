// 条件分岐
pub(crate) fn condion() {
    let x = 100;
    let y = 200;

    if x == y {
        println!("same value");
    };

    // jsで言う三項演算子
    let _z = if x != y { 500 } else { 600 };
}
