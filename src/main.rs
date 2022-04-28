fn main() {
    println!("Hello, world!");
    ownership_test()
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
