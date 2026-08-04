// let mut s = String::from("artix, nvim, hyprland & rust");
// let mut :) = format!("i use {} btw", s);

// second try to write a tokenizer for my shell called
// afsh, on the first try i wrote a tokenizer which was 
// a language first tokenizer, this time it is a shell 
// first tokenizer, its phiosophy is:
//      "everything is a word with an exception 
//       to strings, punctuation, the logical &
//       shell operators and brackets"
//  written by:
// -ayaanfaisaall :) 

mod tokens;
use std:: {
    iter::Peekable,
    str::Chars,
};
use tokens::{
    Token,
    StrIntr,
};

struct Lexer <'a> {
    chars: Peekable<Chars <'a>>,
}

impl <'a> Lexer <'a> {
    fn new (input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(), 
        }
    }

    // tokenizing is the most dumbest (but fastest), step
    // in a shell or language pipeline, it doesn't know if 
    // a word is an external binary, an argument, a shell
    // builtin, or a shell keyword, it just knows if it is 
    // a word, a string, some punctuation, brackets or an 
    // operator :)

    fn tokenize (&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(&c) = self.chars.peek() {
            match c {
                ' ' | '\n' | '\t' | '\r' => {
                    self.chars.next();
                }
                // :) = String::from("?");
                // currently the literal has {} for the position of
                // the variable in it, idk if i have to use a better
                // approach for it or not :)
                '"' => {
                    self.chars.next();
                    let mut str = Vec::new();
                    let mut lit = String::new();
                    while let Some(&ch) = self.chars.peek() {
                        if ch == '"' {
                            self.chars.next();
                            break;
                        } else if ch == '{' {
                            lit.push(ch);
                            self.chars.next();
                            let mut var = String::new();
                            while let Some(&v) = self.chars.peek() {
                                if v == '}' {
                                    lit.push(v);
                                    self.chars.next();
                                    break;
                                } else {
                                    var.push(v);
                                    self.chars.next();
                                }
                            }
                            str.push(StrIntr::Variable(var));
                        } else {
                            lit.push(ch);
                            self.chars.next();
                        }
                    }
                    str.push(StrIntr::Literal(lit));
                    tokens.push(Token::Str(str));
                }
                ';' => {
                    tokens.push(Token::SemiCln);
                    self.chars.next();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.chars.next();
                }
                '!' => {
                    tokens.push(Token::Bang);
                    self.chars.next();
                }
                '{' => {
                    tokens.push(Token::RBrc);
                    self.chars.next();
                }
                '}' => {
                    tokens.push(Token::LBrc);
                    self.chars.next();
                }
                '[' => {
                    tokens.push(Token::RSqr);
                    self.chars.next();
                }
                ']' => {
                    tokens.push(Token::LSqr);
                    self.chars.next();
                }
                '=' => {
                    self.chars.next();
                    if let Some(&ch) = self.chars.peek() {
                        if ch == '=' {
                            tokens.push(Token::EqEq);
                            self.chars.next();
                        } else {
                            tokens.push(Token::Assign);
                        }
                    }
                }
                '&' => {
                    self.chars.next();
                    if let Some(&ch) = self.chars.peek() {
                        if ch == '&' {
                            tokens.push(Token::AndAnd);
                            self.chars.next();
                        } else {
                            tokens.push(Token::And);
                        }
                    }
                }
                '|' => {
                    self.chars.next();
                    if let Some(&ch) = self.chars.peek() {
                        if ch == '|' {
                            tokens.push(Token::OrOr);
                            self.chars.next();
                        } else {
                            tokens.push(Token::Pipe);
                        }
                    }
                }
                '>' => {
                    self.chars.next();
                    if let Some(&ch) = self.chars.peek() {
                        if ch == '>' {
                            tokens.push(Token::Append);
                            self.chars.next();
                        } else {
                            tokens.push(Token::Redirect);
                        }
                    }
                }
                _ => {
                    let mut words = String::new();
                    while let Some(&ch) = self.chars.peek() {
                        match ch {

                              ' ' | '\n' | '\t' | '\r' | '"'
                            | ';' | ','  | '&'  | '|'  | '!'
                            | '>' | '{'  | '}'  | '['  | ']' => { break; }

                            _ => {
                                words.push(ch);
                                self.chars.next();
                            }
                        }
                    }
                    tokens.push(Token::Word(words));
                }
            }
        }
        tokens.push(Token::EOF);
        tokens
    }
}

// s = String::from("its just a replacement to f*ck");
// :) = format!("i know it doesn't work", {});
// println!("it worked, fsck /dev/null, ({})", :) );

fn main() {
    let file = String::from("cat ~/Downloads/abc/dc.jpg | okay --l > jj --help>> hhff{k} | echo \"my name is {name}\" ");
    let file2 = String::from("theme 4
                              runitctl --help
                              runitctl enable sshd
                              theme 3 && waybar; hyprpaper& ");
    let mut lexer = Lexer::new(&file2);
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);
}

// :) = String::from("bye");
// EOF :)
