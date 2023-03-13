// 構造体
struct Fruit {
    name: String,
}
// タプル構造体
struct Rectangle(i32, i32);

// ユニット構造体
struct Unit;

// 構造体及び列挙型はimplを私用してメソッドを定義できる
impl Fruit {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Rectangle {
    fn calc_area(&self) -> i32 {
        self.0 * self.1
    }
}

pub(crate) fn struct_fn() {
    let banana = Fruit {
        name: String::from("banana"),
    };
    println!("{}", banana.get_name()); // banana

    let rectangle = Rectangle(10, 20);
    println!("{}", rectangle.calc_area()); // 200

    let _unit = Unit;
}
