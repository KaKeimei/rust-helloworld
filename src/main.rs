use std::collections::HashMap;
use std::fmt::{Debug, Display, Error, Formatter};
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::ops::Mul;
use std::{io, ops};

use rand::Rng;
use rust_helloworld::eat_at_restaurant;
use rust_helloworld::front_of_house::hosting::seat_at_table;

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

    // 测试特征对象
    trait_object_test();

    // 测试vector
    vector_test();

    // 测试map
    map_test();

    // 测试类型转换
    cast_test();

    // 测试错误处理
    error_test();

    // 测试引入外部依赖
    module_test();
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

trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

// 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
fn draw1(x: Box<dyn Draw>) {
    // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
    x.draw();
}

fn draw2(x: &dyn Draw) {
    x.draw();
}

// 测试特征对象
fn trait_object_test() {
    let x = 1.1f64;
    // do_something(&x);
    let y = 8u8;

    // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    draw1(Box::new(x));
    // 基于 y 的值创建一个 Box<u8> 类型的智能指针
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
    trait_object_exec();
}

trait Bird {
    fn quack(&self) -> String;
}

struct Duck;

impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}

struct Swan;

impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

fn trait_object_exec() {
    // 填空
    let duck = Duck {};
    duck.swim();

    let bird = hatch_a_bird(2);
    // 变成鸟儿后，它忘记了如何游，因此以下代码会报错
    // bird.swim();
    // 但它依然可以叫唤
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // 这只鸟儿忘了如何飞翔，因此以下代码会报错
    // bird.fly();
    // 但它也可以叫唤
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");

    // 当类型不确定时，使用特征对象表示数组
    let birds: [Box<dyn Bird>; 2] = [Box::new(Duck {}), Box::new(Swan {})];

    for bird in birds {
        bird.quack();
        // 当 duck 和 swan 变成 bird 后，它们都忘了如何翱翔于天际，只记得该怎么叫唤了。。
        // 因此，以下代码会报错
        // bird.fly();
    }
}

// 实现以下函数( 返回多个类型）
fn hatch_a_bird(num: i32) -> Box<dyn Bird> {
    if num == 2 {
        return Box::new(Duck {});
    }
    return Box::new(Swan {});
}

fn vector_test() {
    let mut v = vec![1, 2, 3, 4, 5];
    // 使用下标获取元素
    let third = &v[2];
    println!("{}", third);
    // 使用get方法获取元素
    let third_get = v.get(2);
    match third_get {
        None => {}
        Some(i) => {
            println!("{}", i);
        }
    }

    // 在这就会报错，因为在可变引用之后不能存在
    // let first = &v[0];
    v.push(6);
    // 在这就没问题，因为可变引用已经消失
    let first = &v[0];
    println!("{}", first);
    // 在迭代的过程中修改元素的值
    for x in &mut v {
        *x += 10
    }
    for x in v {
        print!("{}, ", x)
    }

    println!();
    // 使用枚举类型来存储不同类型
    let v_enum = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];
    for x in v_enum {
        println!("{:?}", x)
    }

    // 使用特征对象来实现
    let x1 = &IpV4("127.0.0.1".to_string());
    let x2 = &IpV6("::1".to_string());
    let v_trait: Vec<&dyn IpAddrTrait> = vec![x1, x2];
    for x in v_trait {
        x.display();
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

trait IpAddrTrait {
    fn display(&self);
}

struct IpV4(String);

struct IpV6(String);

impl IpAddrTrait for IpV4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}

impl IpAddrTrait for IpV6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}

fn map_test() {
    let team_list = vec![("China", 100), ("America", 40)];
    let team_map: HashMap<&str, i32> = team_list.into_iter().collect();
    println!("{:?}", team_map);

    let name = String::from("Sunface");
    let age = 18;
    let mut handsome_boys = HashMap::with_capacity(1);
    // 直接使用值作为key
    handsome_boys.insert(name, age);

    // 此处不能再使用name，所有权已经转移给了map
    // println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
    // 基础类型没有所有权问题，是值拷贝
    println!("还有，他的真实年龄远远不止{}岁", age);

    // 遍历map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

    // 更新
    scores.insert("Blue".to_string(), 20);
    // 不存在就更新，存在不更新
    scores.entry("Yellow".to_string()).or_insert(5);
    println!("{:?}", scores);

    // 使用 HashMap 来存储 viking 的生命值
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // 使用 derive 的方式来打印 viking 的当前状态
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}

#[derive(PartialEq, Debug, Hash)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

impl Eq for Viking {}

fn cast_test() {
    let a: u8 = 10;
    let b: u16 = 100;

    let b_: u8 = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}

fn error_test() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => {
            drop(file);
        }
        Err(error) => match error.kind() {
            // 选取错误类型
            ErrorKind::NotFound => {
                println!("{:?}", error)
            }
            _ => {
                println!("other error")
            }
        },
    }
    match read_username_from_file() {
        Ok(s) => {
            println!("{}", s)
        }
        Err(err) => {
            println!("{:?}", err)
        }
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s);
    Ok(s)
}

fn module_test() {
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("random {}", secret_number);
    assert_eq!(seat_at_table().as_str(), "sit down please");
    assert_eq!(eat_at_restaurant().as_str(), "yummy yummy!");
}
