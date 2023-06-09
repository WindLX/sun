# 🌞 欢迎使用 Sun 🌞

*version: 0.5.0*

Sun 是一种脚本语言，它具有简洁易读的语法和简单的数据类型，包括 `Nil`、`Bool`、`Number`、`String`、`Table`。

## 1 运行

### 1.1 交互模式

使用 `cargo run` 进入交互模式，使用 `exit()` 来主动退出程序

### 1.2 脚本模式

在命令行添加指定文件的路径: `cargo run /path/to/file`

## 2 数据类型

- `Nil`：表示空值
- `Bool`：表示布尔值，可以是 `true` 或 `false`, 使用 `T` / `F` / `false` / `true` 来创建
- `Number`：表示数值，包括整数和浮点数
- `String`：表示字符串
- `Table`：表示容器数据类型，支持以数组形式和字典形式存储数据

## 3 运算符

- 加减乘除：用 `+`、`-`、`*` 和 `/` 运算符进行加减乘除运算
- 取余：用 `%` 运算符进行取余运算
- 取共轭：用 `*` 运算符进行取共轭运算
- 阶乘：用 `!` 运算符进行阶乘运算
- 幂运算：用 `^` 运算符进行幂运算
- 与或非：用 `&&`、`||` 和 `~` 运算符进行与或非运算
- 异或：用 `^^` 运算符进行异或运算

### 3.! 注意

所有的运算运算当变量名作为第一个位置参数时将改变变量的值，如下：

```sun
// 变量名作为第二个位置参数
i = 0;
1 + i;
print(i)
[o] 0

// 变量名作为第一个位置参数
i = 0;
i + 1;
print(i)
[o] 1
```

使用 `clone()` 或者使用运算符 `'` 获取数据的拷贝以避免引用值的改变

```sun
i = 10;
'i + 1  // i.clone() + 1
```

不过请注意不推荐调用带有返回值函数而不将它们的值赋给新变量，这将会在调用栈上生成垃圾，未来会修复这一问题

```sun
// 赋值语句
a = 0;
b = a + 1;
print(a, b)
[o] 1, 1        // a 的值改变了
c = 'a + 1;
print(a, c)     // a 的值没有改变
[o] 1, 2
d = 1 + a;
print(a, d)
[o] 1, 2        // a 的值没有改变
```

## 4 语句块

使用 `;` 来分割不同的语句

```sun
a = 0; print(a); a = a + 1; print(a)
```

## 5 命令行参数

+ `--ct`: 检查词法分析结果
+ `--cp`: 检查语法树
+ `--cc`: 检查生成的指令
+ `--cs`: 检查调用堆栈
+ `--cg`: 检查全局变量表
+ `--debug`: 查看运行情况

## 6 流程控制

### 6.1 if

使用 `?` 创建条件语句，语法如下：

```sun
? condition: 
    do_1;
    do_2;
    do_3
??
    else_do_1;
    else_do_2
|
```

### 6.2 loop

使用 `$` 创建循环语句，语法如下：

```sun
$ condition: 
    do_1;
    do_2;
    do_3
|
```

`|` 表示语句块的结束，conditon 可以是比较语句的组合: `a > 0 && b < 2 || (c + 1) > 3`

## 7 类型方法

使用 `.` 获取某值的方法

```sun
a = 10;
b = a.add(20);
print(b)
[o] 30
ADD = a.add;
print(ADD)
[o] <function: 0xd6d530>
c = ADD(40, 2)
print(c)
[o] 42
```

实际上所有运算符都是实现了该运算符运算的类型方法

## 8 系统方法

一些对虚拟机的操作方法

+ `drop` 接收一个 `String` 类型的参数，从全局变量表中删除指定变量
+ `show` 接收一个 `String` 类型的参数，打印指定信息，例如：使用 `global` 来打印全局变量表的内容

## 9 元调用

使用 `TypeName::method_name` 语法直接调用元方法，例如：

```sun
a = -10
a = Math::abs(a)
print(a)
[o] 10
```

## 10 Table 数据类型

Table 是一种容器数据类型，支持以数组形式和字典形式存储数据。您可以使用以下初始化语句来创建 Table：

```sun
{10, "a", false, "key": 100}
```

这将创建一个包含 10、"a" 和 false 这些值的数组，以及一个 key 为 "key"，值为 100 的键值对。您可以使用以下方法来访问 Table 中的数据：

- 通过下标访问数组元素：`table[1]`
- 通过键访问字典元素：`table["key"]`
- table 的其他方法：`remove` `push` `insert` `extend` `aextend` `dextend` `alen` `dlen` `len`

## 11 类型转换

对于标准库中的 Class 类型，基本都实现了转换成 Table 的元方法：

```sun
cpx = Math::cpx(1, 1)
>>cpx
print(cpx)
[o]
cpx = Complex::from(cpx)
print(cpx)
[o]
```

## 12 示例

以下是一些 Sun 语言的示例代码：

```sun
// 定义一个变量
x = 10

// 创建一个 Table
table = {10, "a", false, "key": 100}

// 访问 Table 中的元素
print(table[1])
print(table["key"])

// 修改 Table
table.insert("new_key", x)
table.insert(0, "at_0")
table.remove(1)
print(table)

new_table = {0, "c", false}
new_table.extend(table)
print(table)

// 循环
i = 0;
t = {10, 20, 30, 40};
$ i <= 3: print(t[i]); i = i + 1 | 
[o] 10
[o] 20
[o] 30
[o] 40
```

## 12 TODO

- [x] 流程控制
- [x] 语句块
- [x] 元调用
- [ ] 自定义函数
- [ ] 自定义类
- [x] Rust Api
- [x] 完善代码注释
- [ ] 文档系统
- [ ] 调试系统
- [ ] 闭包
- [ ] 协程
- [ ] 完善标准库
- [ ] VsCode 插件