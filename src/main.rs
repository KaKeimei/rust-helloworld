fn main() {
    println!("Hello, world!");
    // 测试所有权转移和归还
    ownership_test();

    // 测试引用与解引用
    reference_test();
}

fn ownership_test() {
    let mut s1 = String::from("hello");
    // 函数调用将转移所有权，返回值会交还所有权
    s1 = takes_ownership_and_give_back(s1);
    println!("s1 = {}", s1);
}

fn takes_ownership_and_give_back(some_string: String) -> String{
    print!("{}", some_string);
    return some_string
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
fn change_string(some_string:&mut String) {
    some_string.push_str(", world");
}