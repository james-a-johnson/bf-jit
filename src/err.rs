use thiserror::Error;

#[derive(Debug, Error)]
pub enum BFError {
    #[error("Encountered an invalid instruction {0}")]
    Instruction(char),
    #[error("Encountered an unmatched bracket")]
    Unmatched,
    #[error("Encountered an IO error during execution")]
    IO(#[from] std::io::Error),
}