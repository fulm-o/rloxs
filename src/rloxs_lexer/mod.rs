pub mod lexer;
mod errors;

#[cfg(test)]
mod tests;

pub use lexer::Lexer;
pub use errors::LexerError;