use mcro_demo::*;

#[foo_bar_attribute]
struct MyStruct {
    baz: i32,
    baz1: i32,

    baz2: i32,

    baz3: i32,

}

fn main() {
    let demo = MyStruct::default();
    println!("struct is {:?}", demo);
    let double_foo = demo.double_foo();
    println!("double foo: {}", double_foo);
}