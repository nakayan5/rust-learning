// rustではクラスとメソッドと同等の機能はstruct/enumとimplで実装可能だが、継承はできない
// それをできるようにしたのがtrait

trait Greeter {
    fn greet(&self);
}

struct Person(String);

impl Greeter for Person {
    fn greet(&self) {
        println!("Hello {}", self.0);
    }
}

// deriveアトリビュートを記述して型に標準的な実装を追加できるtrait
// アトリビュートはマクロの一種で、コンパイラがその型用の実装を自動的に算出する
#[derive(Debug)]
struct Hours(u32);

fn main() {
    let h = Hours(5);
    println!("{:?}", h); // => Hours(5)
}
