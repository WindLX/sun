# 欢迎使用 Sun 语言 🌞

Sun 是一种脚本语言，它具有简洁易读的语法和简单的数据类型，包括 `Nil`、`Bool`、`Number`、`String`、`Table`。

## 运行

#### 交互模式

`cargo run`

#### 脚本模式

在命令行添加指定文件的路径: `cargo run /path/to/file`

## 数据类型

- `Nil`：表示空值
- `Bool`：表示布尔值，可以是 `true` 或 `false`, 使用 `T` / `F` / `false` / `true` 来创建
- `Number`：表示数值，包括整数和浮点数
- `String`：表示字符串
- `Table`：表示容器数据类型，支持以数组形式和字典形式存储数据

## 运算符

- 加减乘除：用 `+`、`-`、`*` 和 `/` 运算符进行加减乘除运算
- 取余：用 `%` 运算符进行取余运算
- 取共轭：用 `*` 运算符进行取共轭运算
- 阶乘：用 `!` 运算符进行阶乘运算
- 幂运算：用 `^` 运算符进行幂运算
- 与或非：用 `&&`、`||` 和 `~` 运算符进行与或非运算
- 异或：用 `^^` 运算符进行异或运算

注意: 所有的运算运算当变量名作为第一个位置参数时将改变变量的值，如下：
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

## 语句块

使用 `;` 来分割不同的语句

```sun
a = 0; print(a); a = a + 1; print(a)
```

## 命令行参数
TODO:

## 流程控制

#### if

使用 `?` 创建条件语句，语法如下：

```sun
? condition: 
    do_1;
    do_2;
    do_3
??: 
    else_do_1;
    else_do_2
|
```

#### loop

使用 `$` 创建循环语句，语法如下：

```sun
$ condition: 
    do_1;
    do_2;
    do_3
|
```

`|` 表示语句块的结束，conditon 可以是比较语句的组合: `a > 0 && b < 2 || (c + 1) > 3`

## Table 数据类型

Table 是一种容器数据类型，支持以数组形式和字典形式存储数据。您可以使用以下初始化语句来创建 Table：

```sun
{10, "a", false, "key": 100}
```

这将创建一个包含 10、"a" 和 false 这些值的数组，以及一个 key 为 "key"，值为 100 的键值对。您可以使用以下方法来访问 Table 中的数据：

- 通过下标访问数组元素：`table[1]`
- 通过键访问字典元素：`table["key"]`

## 示例

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

```

## TODO

- [x] 流程控制
- [x] 语句块
- [ ] 自定义函数
- [ ] 自定义类
- [ ] Rust Api
- [x] 完善代码注释
- [ ] 文档系统
- [ ] 调试系统
- [ ] 协程
- [ ] 完善标准库