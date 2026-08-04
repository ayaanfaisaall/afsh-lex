//
// note: () or parenthesis are intentionally not 
// allowed in my shell and its language and hence
// (), don't even have a Token
//
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // general
    Word(String),
    Str(Vec<StrIntr>),
    // punctuation
    SemiCln,
    Colon,
    Comma,
    And,
    // logical operators
    Assign,
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
    // lang keywords
    Let,
    Print,
    If,
    Elif,
    Else,
    For,
    While,
    Break,
    // equality operators
    EqualTo, 
    LessEqual, 
    LessThan,
    GreaterEqual,
    GreaterThan,
    // end of file
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StrIntr {
    Literal(String),
    Variable(String),
}
