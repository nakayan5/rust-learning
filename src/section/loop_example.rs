// 繰り返し処理
pub(crate) fn loopFn() {
    for i in 0..10 {
        println!("in for-loop: {}", i)
    }

    let mut count = 0;
    while count < 10 {
        // count++みたいな書き方はできない
        count += 1;
    }

    loop {
        count -= 1;
        if count == 0 {
            break;
        }
    }
}
