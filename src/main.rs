mod section;

use rand::random;

fn main() {
    // let str_slice: &str = "world";
    // grammer()\
    let random_numbner: i32 = random();
    println!("{}", random_numbner);

    if true == true {
        println!("trueです！")
    }

    section::enum_example::color_fn();
    section::struct_example::struct_fn();
    section::condion_example::condion();
    section::loop_example::loopFn();
    section::tuple_example::tuple();
    section::test_example::div(10, 100);
}
