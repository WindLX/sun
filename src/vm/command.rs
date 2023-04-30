use crate::sun_lib::sun_value::SunValue;

/*
    Sun 虚拟机的指令集
*/
#[derive(Debug)]
pub enum Command {
    /*
        从全局变量表加载值到栈上
        para:
            para_0: String 变量名
    */
    LoadValue(String),

    /*
        将栈上值加载到全局变量表中
        para:
            para_0: String 变量名
    */
    SetGlobalValue(String),

    /*
        直接加载值到栈上
        para:
            para_0: SunValue 值
    */
    AddValue(SunValue),

    /*
        按索引查询Table，将结果加载到栈上
    */
    LoadTableValueByIndex,

    /*
        按键查询Table，将结果加载到栈上
    */
    LoadTableValueByKey,

    /*
        按TableIndex为Table赋值
    */
    SetTableValue,

    /*
        从全局变量表加载函数到调用栈上
        para:
            para_0: String 函数名
    */
    LoadFunc(String),

    /*
        调用函数
        para:
            para_0: u8 参数个数
    */
    Call(u8),
}
