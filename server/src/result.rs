// 自定义错误类型的返回结果
use::std::result;
pub type Result<T> = result::Result<T,String>;