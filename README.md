# 欢迎使用 Sun 语言 🌞

Sun 是一种脚本语言，它具有简洁易读的语法和简单的数据类型，包括 `Nil`、`Bool`、`Number`、`String`、`Table`。

## 数据类型

- `Nil`：表示空值
- `Bool`：表示布尔值，可以是 `true` 或 `false`
- `Number`：表示数值，包括整数和浮点数
- `String`：表示字符串
- `Table`：表示容器数据类型，支持以数组形式和字典形式存储数据

## 运算符

- 加减乘除：用 `+`、`-`、`*` 和 `/` 运算符进行加减乘除运算
- 取余：用 `%` 运算符进行取余运算
- 取共轭：用 `*` 运算符进行取共轭运算
- 阶乘：用 `!` 运算符进行阶乘运算
- 幂运算：用 `^` 运算符进行幂运算
- 与或非：用 `and`、`or` 和 `not` 运算符进行与或非运算
- 异或：用 `xor` 运算符进行异或运算

## Table 数据类型

Table 是一种容器数据类型，支持以数组形式和字典形式存储数据。您可以使用以下初始化语句来创建 Table：

```
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
table.remove(1)
print(table)

new_table = {0, "c", false}
new_table.extend(table)
print(table)

```