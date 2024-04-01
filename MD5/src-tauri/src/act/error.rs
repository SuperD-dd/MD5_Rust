use serde_json::error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MD5Error {
    #[error("打开文件失败")]
    OpenFileError,
    #[error("请求数据错误")]
    ReqwestError,
}

impl serde::Serialize for MD5Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
