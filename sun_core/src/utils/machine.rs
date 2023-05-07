use crate::utils::SunPointer;

pub trait IsMachine {
    /// 从栈中取出值
    fn pop(&mut self) -> Option<SunPointer>;

    /// 从全局变量表删除值
    fn drop(&mut self, name: &str);

    /// 获取元数据中的元方法名
    fn get_meta(&self, name: &str) -> Option<Vec<&str>>;

    /// 打印全局变量
    fn show_global(&self);
}
