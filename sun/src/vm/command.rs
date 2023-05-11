use sun_core::container::SunValue;

/// Sun 虚拟机的指令集
#[derive(Debug)]
pub enum Command {
    /**
        从全局变量表加载值到栈上
        + `name`: `String` 变量名
    */
    LoadValue(String),

    /**
        将栈上值存储到全局变量表中
        + `name`: `String` 变量名
    */
    StoreGlobal(String),

    /**
        直接加载值到栈上
        + `value`: `SunValue` 值
    */
    LoadConst(SunValue),

    /**
        获取对象的方法
        + `method_name`: `String` 方法名
    */
    LoadMethod(String),

    /**
        创建表的指令
        + `number`: `usize` 内容个数
    */
    CreateTable(usize),

    /**
        创建键值对指令
        + `key_name`: `String` 键名
    */
    SetPair(String),

    /**
        处理对Table的赋值
    */
    SetTable,

    /**
        调用函数
        + `para_number`: `usize` 参数个数
    */
    Call(usize),

    /*
        条件跳转
        + `jump`: `usize` 跳转的位置的偏移
    */
    TestJump(usize),

    /*
        无条件跳转
        + `jump`: `usize` 跳转的位置偏移
    */
    Jump(usize),

    /*
        无条件反向跳转
        + `jump`: `usize` 跳转的位置偏移
    */
    Back(usize),

    /*
        导入模块
        + `lib_name`: `String` 模块的地址
    */
    Import(String),

    /*
        元调用
        + `meta_name`: `String` 类型名称
        + `method_name`: `String` 方法名
    */
    LoadMetamethod(String, String),
}
