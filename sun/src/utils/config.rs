/// 运行配置
#[derive(Debug)]
pub struct Config {
    /// `check_tokenizer`: `--ct` 检查词法分析器
    pub check_tokenizer: bool,
    /// `check_parser`: `--cp` 检查语法树
    pub check_parser: bool,
    /// `check_command`: `--cc` 检查虚拟器当前执行的命令
    pub check_command: bool,
    /// `check_stack`: `--cs` 检查调用堆栈
    pub check_stack: bool,
    /// `check_global`: `--cg` 检查全局变量表
    pub check_global: bool,
    /// `is_debug`: `--debug` 检查虚拟器运行信息
    pub is_debug: bool,
}

impl Config {
    /// 构建配置的结构体
    pub fn new() -> Self {
        Config {
            check_tokenizer: false,
            check_parser: false,
            check_command: false,
            check_stack: false,
            check_global: false,
            is_debug: false,
        }
    }
}
