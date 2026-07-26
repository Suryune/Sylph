use thiserror::Error;

/// 定义错误别名,方便错误处理
pub type Result<T> = std::result::Result<T, Error>;

/// 处理各种错误
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO 操作失败: {0}")]
    Io(#[from] std::io::Error),

    #[error("序列化/反序列化失败: {0}")]
    Rkyv(#[from] rkyv::rancor::Error),
}
