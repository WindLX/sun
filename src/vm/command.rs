use crate::vm::value::SunValue;

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
        向全局变量表插入值
        para:
            para_0: String 变量名
            para_1: SunValue 变量值
    */
    SetValue(String, SunValue),

    /*
        复制全局变量表值至另一个值
        para:
            para_0: String 被复制的键
            para_1: String 目标键
    */
    CopyValue(String, String),

    /*
        从全局变量表加载函数到栈上
        para:
            para_0: String 函数名
    */
    LoadFunc(String),

    /*
        创建Tensor
    */
    CreateTensor,

    /*
        为Tensor指定索引值赋值
        para:
            para_0: u8 Tensor的索引值
    */
    SetTensor(u8),

    /*
        调用函数
        para:
            para_0: u8 参数个数
    */
    Call(u8),
}
