use crate::{ast::IdentifierExpression, prelude::*};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum IdentifierAnnotation {
    Dot(Vec<IdentifierExpression>),
    Name(IdentifierExpression),
}

/// <https://www.pyret.org/docs/latest/s_annotations.html>
#[derive(Debug, PartialEq)]
pub enum AnnotationType {
    /// <https://www.pyret.org/docs/latest/s_annotations.html#(part._s~3aname-ann)>
    NameAnnotation {
        value: IdentifierAnnotation,
        /// <https://www.pyret.org/docs/latest/s_annotations.html#(part._s~3aapp-ann)>
        parameters: Vec<IdentifierExpression>,
        /// <https://www.pyret.org/docs/latest/s_annotations.html#(part._s~3apred-ann)>
        predicate: Option<IdentifierExpression>,
    },
    /// <https://www.pyret.org/docs/latest/s_annotations.html#(part._s~3aarrow-ann)>
    ArrowAnnotation {
        generics: Option<Vec<IdentifierExpression>>,
        arguments: Vec<AnnotationType>,
        return_annotation: Box<AnnotationType>,
    },
    /// <https://www.pyret.org/docs/latest/s_annotations.html#(part._s~3atuple-ann)>
    TupleAnnotation(Vec<AnnotationType>),
    /// <https://www.pyret.org/docs/latest/s_annotations.html#(part._s~3arecord-ann)>
    RecordAnnotation(HashMap<IdentifierExpression, AnnotationType>),
}

#[derive(Leaf, Debug, PartialEq)]
#[regex("::")]
pub struct TypeAnnotation {
    span: (usize, usize),
    pub value: AnnotationType,
}

impl TokenParser for TypeAnnotation {
    #[inline]
    fn parse(_input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let start_position = state.next_position;

        let end;

        let value = match state.lex::<IdentifierExpression>()? {
            Some(ident) => {
                end = ident.end();

                AnnotationType::NameAnnotation {
                    value: IdentifierAnnotation::Name(ident),
                    parameters: vec![],
                    predicate: None,
                }
            }
            None => {
                todo!()
            }
        };

        Ok(Self {
            span: (start_position, end),
            value,
        })
    }
}