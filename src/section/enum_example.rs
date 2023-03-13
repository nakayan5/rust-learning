// 列挙型
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

impl Color {
    fn color_code(&self) -> String {
        match self {
            Color::Red => String::from("#ff0000"),
            Color::Green => String::from("#00ff00"),
            Color::Blue => String::from("#0000ff"),
            Color::Custom(r, g, b) => {
                format!("{:02x}{:02x}{:02x}", r, g, b)
            }
        }
    }
}

pub(crate) fn color_fn() {
    let color = Color::Blue;
    println!("{}", color.color_code());

    let color = Color::Custom(10, 123, 255);
    println!("{}", color.color_code());
}
