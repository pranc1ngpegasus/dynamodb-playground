#[derive(Debug)]
pub struct PersistError {
    pub desc: String,
}

impl std::fmt::Display for PersistError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PersistError: {}", self.desc)
    }
}

impl std::error::Error for PersistError {}
