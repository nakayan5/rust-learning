// 値が所有されている期間をライフタイムという。

pub(crate) fn ownership_fn() {
    {
        let x = String::from("hello");
        let y = x;
        // 所有権が移動する

        // xの所有権はyに移動済みなのでxは使えない
        // println!("{}", x);
        println!("{}", y);
    }
    // ここでyも解放される

    {
        let z = String::from("hello");
        {
            let w = &z;

            println!("w: {}", w)
        }
        // ここでwも解放される
        // zはまだ使用可能

        println!("z: {}", z)
    }
    // ここでzも解放される
}

// 'a がライフタイムパラメータ
struct Greet<'a> {
    word: &'a str,
}

impl<'a> Greet<'a> {
    fn say(&self) {
        println!("{}", self.word);
    }
}

fn greet_fn() {
    let hello = "hello";

    {
        let greet = Greet { word: hello };
        greet.say();
    }
    // greetはここで解放

    println!("{}", hello);
}
// helloはここで解放
