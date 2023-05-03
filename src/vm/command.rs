use crate::sun_lib::value::sun_object::SunValue;

/*
    Sun 虚拟机的指令集
*/
#[derive(Debug)]
pub enum Command {
    /*
        从全局变量表加载值到栈上
        para:
            para_0: &str 变量名
    */
    LoadValue(String),

    /*
        将栈上值存储到全局变量表中
        para:
            para_0: &str 变量名
    */
    StoreGlobal(String),

    /*
        直接加载值到栈上
        para:
            para_0: SunValue 值
    */
    LoadConst(SunValue),

    /*
        获取对象的符号method
        para:
            para_0: &str 方法名
    */
    LoadMethod(String),

    CreateTable(u8),

    SetPair(String),

    /*
        处理对Table的赋值
    */
    SetTable,

    /*
        调用函数
        para:
            para_0: u8 参数个数
    */
    Call(u8),
}
