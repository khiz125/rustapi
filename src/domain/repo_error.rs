/// リポジトリ操作で発生しうるエラーの抽象定義
/// 具体的なDB実装の詳細は含まない
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("database error: {0}")]
    DatabaseError(String),

    #[error("data integrity error: {0}")]
    DataIntegrityError(String),
}
