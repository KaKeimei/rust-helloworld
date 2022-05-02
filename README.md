# rust helloworld

个人rust学习练习项目。

教程： https://course.rs/

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

#### 元组

多种类型组合在一起的，长度固定，元组中的元素顺序也是固定的。

模式匹配解构元组：用同样的形式把一个复杂对象中的值匹配出来。

#### 结构体

必须将结构体声明为可变的，才能修改其中的字段，rust不支持将某个结构体某个字段标记为可变。

初始化结构体时，必须初始化每个字段。

结构体可以简化创建，变量名和结构体名相同时，可以直接使用。

更新结构体的时候，可以将已有结构体传入新结构体内部来简化更新。结构体更新赋值的时候，可能会转移所有权。

元组结构体，字段没有名称，长得很像元组。

单元结构体类似单元类型。没有任何字段和属性。

在结构体中使用引用，必须加上生命周期标识符。

#### 枚举
枚举类型是一个类型，会包含每一个可能的枚举成员，而枚举值是该类型中的某个成员的实例。

枚举成员还可以直接关联数据，不仅如此，同一个枚举类型下的不同成员还可以持有不同的数据类型（Java不允许这么做）。可以同一化类型。

Option枚举用来处理空值，rust抛弃了null值，而改为使用Option枚举变量来表述这种结果，Option有两个成员，一个是Some表示有值，一个是None表示没有值。

Some和None无需使用Option:: 就可以使用。通过Some包装后的值，不能直接与原值进行交互，因为属于不同的类型，这样的包装有利于解决隐藏的空指针问题。要进行运算必须转换类型为T（各种unwrap方法）

Match表达式可以使用匹配模式识别枚举的成员，根据不同的成员使用不同的逻辑。

#### 数组
Array 速度很快但长度固定， Vector（动态数组）可以动态增长但是有性能损耗。

Array和Vector的关系与&str和String 的关系很像，前者是固定长度的字符串切片，后者是可动态增长的字符串。

数组必须满足 长度固定，类型相同，线性排列这三个条件，因此array是可以储存在栈内存上的。

数组切片，允许引用部分连续的片段，使用切片引用类型&[T]

### 流程控制

#### If 分支控制

if语句块是表达式，可以给变量赋值。

#### 循环控制

for in 循环：

```rust
for item in &container {

}
```

使用for的时候，往往使用的是集合的引用形式，除非不想在后续代码中使用该集合（所有权转移后回收）。

while vs for: for不会使用索引去访问数组，因此更加安全和简洁，同时避免数组边界检查，性能更好。

loop循环：适用所有循环场景，就是一个简单的无限循环，使用时一定要注意死循环影响性能。

### 模式匹配

模式匹配经常出现在函数式编程中，用于为复杂的类型系统提供轻松的解构能力。

可以使用match来替代else if这种丑陋的多重分支使用方式。

match匹配enum类似于其他语言的switch，而默认情形_, 类似于default。

match本身也可以是一个表达式，用它进行赋值。

模式匹配的一个重要功能是从模式中取出绑定的值（比如取出enum中绑定的值）。

matches!宏，可以返回匹配结果为true或者false

匹配模式同名变量可以覆盖老的值，绑定新的值（使用匹配守卫语句可以避免老的值被覆盖）。

匹配模式可以用来解构Option，解决rust变量是否有值的问题。

@(at)绑定。允许在模式中将模式中的一个字段绑定另外一个变量。

### 方法

rust的方法定义使用的是struct和impl分开的形式（类似于go），可以给予使用者极高的灵活度。

#### self &self &mut self

&self 是 self: &Self 的简写，在impl块内，Self指代被实现方法的结构体类型，而self指代此类型的实例。

self 依然有所有权的概念：
1. self表示所有权转移到该方法中，这种形式用的很少
2. &self表示对类型的不可变借用
3. &mut self表示可变借用

self的使用跟函数参数一样，要严格遵守所有权规则。

使用方法代替函数有以下好处：

* 不用在函数签名中重复些类型
* 代码的组织性和内聚性更强，方便代码维护阅读。

方法名允许与字段名相同，主要用于实现getter访问。

rust对于方法接受者使用隐式借用，因此不需要像c或c++一样使用两个不同的运算符来调用方法(. 和 ->)。当object.something()调用方法时，rust会自动为object添加&，&mut或者*以便是object与方法的签名匹配。

#### 关联函数

定义在impl中但没有self的函数称为关联函数，因为没有self，所以不能使用f.read()的用法调用，因此它是函数而不是方法。（类似java中的静态函数？）

rust 有一个约定俗成的规则，使用new来作为构造器的名称。

使用:: 来调用关联方法，该方法位于结构体的命名空间中，:: 语法用于关联函数和模块创建的命名空间。

同一个struct可以有多个impl定义。

枚举类型也可以实现方法。

### 泛型和特征

用同一功能的函数处理处理不同类型的数据。在不支持泛型的语言中通常需要为每一种类型编写一个函数。

T 是泛型参数，泛型参数可以用任意字母。使用泛型参数前必须对其进行声明：

```rust
fn largest<T>(list: &[T]) -> T {}
```

结构体中也可以使用泛型，跟泛型函数类似，使用泛型之前必须声明泛型参数Point<T>.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

枚举中也可以使用泛型

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

```

方法中也可以使用泛型。

不仅可以定义基于T的方法，还可以针对特定的具体类型来进行方法定义。

Rust泛型是零成本抽象，在编译的时候对每个类型生成各自的代码，损失了编译速度并增大了最终的文件大小。但是好处就是没有性能上的损失。

#### const 泛型

针对值的泛型，在数组类型中，不同长度的数组其实代表不同的类型，为了解决不同长度数组的问题，引入了


