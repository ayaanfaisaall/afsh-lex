#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // general
    Word(String),
    Str(Vec<StrIntr>),
    // punctuation
    SemiCln,
    Comma,
    And,
    // logical operators
    AndAnd,
    OrOr,
    Bang,
    // shell operators
    Pipe,
    Redirect,
    Append,
    // brackets
    RBrc,
    LBrc,
    RSqr,
    LSqr,
    // end of file
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StrIntr {
    Literal(String),
    Variable(String),
}
