# rust helloworld

个人rust学习练习项目。

## 笔记总结

### 所有权与借用

引用分可变引用与不可变引用，只有可变引用才可以修改对应的值。

引用的引用的作用域从创建开始，一直持续到它最后一次使用的地方，这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号 。

可变引用同时只能存在一个（一个可变引用的作用域内，不能存在指向同一个变量的另一个可变引用）

可变引用与不可变引用不能同时存在

编译器会确保数据不会在其引用之前被释放，想释放数据，必须停止使用引用。避免了悬垂引用的问题（dangling reference）

* 同一时刻要么拥有一个可变引用，要么拥有任意多个不可变引用
* 引用必须是有效的

### 复合类型

只使用基本类型的局限性：无法从更高层次来简化代码。

字符串（String）以及字符串切片（&str）。字面量字符串本质是切片。字符串使用UTF-8编码。Rust不允许索引字符串。对字符串进行切片非常危险，无法保证索引的字节正好位于字符的边界上。

字面值str不可变，性能快速且高效。而String类型是程序运行中生成的，大小不可预知，需要请求内存存放string，并且使用完成后要归还。

你只能将 String 跟 &str 类型进行拼接，并且 String 的所有权在此过程中会被 move.

与 str 的很少使用相比，&str 和 String 类型却非常常用，因此也非常重要。

> 对于 Rust 而言，安全和性能是写到骨子里的核心特性，如果使用 GC，那么会牺牲性能；如果使用手动管理内存，那么会牺牲安全，这该怎么办？为此，Rust 的开发者想出了一个无比惊艳的办法：变量在离开作用域后，就自动释放其占用的内存


