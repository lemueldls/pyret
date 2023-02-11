use crate::{
    error::{PyretResult, SerializedToken},
    LexerState,
};
use std::{fmt::Debug, ops::Range};

pub trait Token: Debug + PartialEq + Sized {
    const NODE_NAME: &'static str;

    fn leaf_name(&self) -> &str;

    fn start(&self) -> usize;

    fn end(&self) -> usize;

    #[inline]
    fn span(&self) -> Range<usize> {
        self.start()..self.end()
    }

    fn serialize(&self) -> SerializedToken {
        SerializedToken {
            name: Box::from(self.leaf_name()),
            span: self.start()..self.end(),
        }
    }
}

pub trait TokenLexer: Token {
    /// # Errors
    ///
    /// Will return an [`Error`] if there was an error parsing the token.
    ///
    /// [`Error`]: crate::Error
    fn lex(state: &mut LexerState) -> PyretResult<::std::option::Option<Self>>;
}

pub trait TokenParser: Token {
    /// # Errors
    ///
    /// Will return an [`Error`] if there was an error parsing the token.
    ///
    /// [`Error`]: crate::Error
    fn parse(input: Box<str>, state: &mut LexerState) -> PyretResult<Self>;
}
