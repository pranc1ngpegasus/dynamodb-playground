#[derive(Debug)]
pub struct ModelError {
    pub desc: String,
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModelError: {}", self.desc)
    }
}

impl std::error::Error for ModelError {}
