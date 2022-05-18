use std::fmt::{Debug, Display, Error, Formatter};
use std::ops;
use std::ops::Mul;

fn main() {
    println!("Hello, world!");
    // 测试所有权转移和归还
    ownership_test();

    // 测试引用与解引用
    reference_test();

    // 测试字符串
    string_test();

    // 测试三元组
    turple_test();

    // 测试结构体
    struct_test();

    // 测试枚举类型
    enum_test();

    // 测试数组
    array_test();

    // 测试模式匹配
    match_test();

    // 测试方法
    method_test();

    // 测试泛型
    generic_test();

    // 测试特征
    trait_test();
}

fn ownership_test() {
    let mut s1 = String::from("hello");
    // 函数调用将转移所有权，返回值会交还所有权
    s1 = takes_ownership_and_give_back(s1);
    println!("s1 = {}", s1);
}

fn takes_ownership_and_give_back(some_string: String) -> String {
    print!("{}", some_string);
    return some_string;
}

fn reference_test() {
    let x = 5;
    // 将x的引用赋值给y
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let mut s1 = String::from("hello");
    // 此处传入的是s1的引用，而不是s1本身，引用是栈内存的，不涉及所有权转移，因此s1也是可以继续使用的
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);
    // 可变引用
    change_string(&mut s1);

    // 可变引用的作用域是从创建到最后一次使用为止(栈内存)，与变量的作用域不同

    // 可变引用同时不能存在两个
    // let ref_s1 = &mut s1;
    // let ref2_s1 = &mut s1;
    // println!("{}, {}", ref_s1, ref2_s1);

    // 可变引用与不可变引用不能同时存在
    // let mut s = String::from("hello");
    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题
    // println!("{}, {}, and {}", r1, r2, r3);
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

// 不可变变量的引用也是不可变的，想要通过引用改变值，就必须让引用是可变变量
fn change_string(some_string: &mut String) {
    some_string.push_str(", world");
}

fn string_test() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    // 可以直接加str，但是不能直接加String，需要borrow
    s += &"!".to_string();
    println!("{}", s);

    // 你只能将 String 跟 &str 类型进行拼接，并且 String 的所有权在此过程中会被 move
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + s2.as_str();
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);

    // 有时候需要转义的字符很多，我们会希望使用更方便的方式来书写字符串: raw string.
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");
    // 如果你希望在字符串中使用双引号，可以使用以下形式
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);
    // 如果希望在字符串中使用 # 号，可以如下使用：
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);
    // 填空
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    // 填空，打印出 "你好，世界" 中的每一个字符
    for c in "你好，世界".chars() {
        println!("{}", c);
    }
}

fn turple_test() {
    // 使用. 来访问元组
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);

    // 使用模式匹配来解构
    let (i, f, u) = x;
    println!("{}, {}, {}", i, f, u);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn struct_test() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // 打印结构体，结构体必须实现Display或者Debug特征
    println!("rect1 is {:?}", rect1);
    println!("{}, {}", rect1.width, rect1.height);
    // 使用dbg宏，可以打印详细信息以及行号
    dbg!(&rect1);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

fn enum_test() {
    // 枚举类型的不同成员可以是不同的类型
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    // todo 获取枚举成员内部的数据
    let m3 = Message::ChangeColor(255, 255, 0);
    println!("{:?}, {:?}, {:?}", m1, m2, m3)
}

fn array_test() {
    // 编译器自动推导出one的类型
    let one = [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    // 借用arrays的元素用作循环中
    for a in &arrays {
        print!("{:?}: ", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn match_test() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    // 使用模式匹配来取出绑定值
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            // Action::ChangeColorRGB(r, g, _) => {
            //     println!(
            //         "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
            //         r, g,
            //     );
            // }
            // 通配符，返回单元类型，什么也不发生
            _ => (),
        }
    }

    // 测试匹配值是否存在
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}, none {:?}", six, none)
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn method_test() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let m = Message::Quit;
    m.call();
    m.call_again()
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

// impl 可以定义多个
impl Message {
    fn call_again(&self) {}
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // 结构体，方法也可以使用泛型
    fn x(&self) -> &T {
        return &self.x;
    }
}

// 测试泛型
fn generic_test() {
    let p = Point { x: 5, y: 10 };

    println!("{}, {}", p.x(), p.y);
}

pub trait Summary {
    fn summarize(&self) -> String;
    fn default_summary(&self) -> String {
        String::from("(read more ...)")
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章是{}, 作者是{}", self.title, self.content)
    }
}

impl Display for Post {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return Err(Error);
    }
}

fn trait_test() {
    let post = Post {
        title: "Rust 语言介绍".to_string(),
        author: "Sunface".to_string(),
        content: "Rust 棒极了".to_string(),
    };
    println!("{}", post.summarize());
    println!("{}", post.default_summary());
    notify(&post);
    notify_constrain(&post);
    trait_exercise()
}

fn notify(item: &impl Summary) {
    println!("Breaking news {}", item.summarize());
}

fn notify_constrain<T: Summary + Display>(item: &T) {
    println!("Breaking news {}", item.summarize());
}

trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student {}

impl Hello for Student {
    fn say_hi(&self) -> String {
        return "hi".to_string();
    }

    fn say_something(&self) -> String {
        return "I'm a good student".to_string();
    }
}

struct Teacher {}

impl Hello for Teacher {
    fn say_hi(&self) -> String {
        return "Hi, I'm your new teacher".to_string();
    }

    fn say_something(&self) -> String {
        return "I'm not a bad teacher".to_string();
    }
}

fn multiply<T: Mul<T, Output = T>>(a: T, b: T) -> T {
    return a * b;
}

struct Foo;

struct Bar;

#[derive(PartialEq, Debug)]
struct FooBar;

#[derive(PartialEq, Debug)]
struct BarFoo;

// 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

impl ops::Sub<Bar> for Foo {
    type Output = BarFoo;

    fn sub(self, _rhs: Bar) -> BarFoo {
        BarFoo
    }
}

fn trait_exercise() {
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!");

    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success!");

    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);
    example1();
    // example2();
}

fn example1() {
    // `T: Trait` 是最常使用的方式
    // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(10), 11);
    assert_eq!(cacher.value(15), 11);
}
