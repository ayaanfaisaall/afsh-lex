//
// let mut s = String::from("artix, nvim, hyprland & rust");
// let mut :) = format!("i use {} btw", s);
//
// second try to write a tokenizer for my shell called
// afsh, on the first try i wrote a tokenizer which was 
// a language first tokenizer, this time it is a shell 
// first tokenizer, its phiosophy is:
//
//      "everything is a word with an exception 
//       to strings, punctuation, logical and 
//       the shell operators and the brackets"
//
//  written by:
// -ayaanfaisaall :) 
//
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
    //
    // tokenizing is the most dumbest (but fastest), step
    // in a shell or language pipeline, it doesn't know if 
    // a word is an external binary, an argument, a shell
    // builtin, or a shell keyword, it just knows if it is 
    // a word, a lang keyword, a string, some punctuation,
    // brackets or an operator :)
    //
    fn tokenize (&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(&c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\r' => {
                    self.chars.next();
                }
                '\n' => {
                    tokens.push(Token::NewLine);
                    self.chars.next();
                }
                //
                // :) = String::from("?");
                // currently the literal has {} for the position of
                // the variable in it, idk if i have to use a better
                // approach for it or not :)
                //
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
                    tokens.push(Token::Assign);
                    self.chars.next();
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.chars.next();
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
                '/' => {
                    self.chars.next();
                    if let Some(&ch) = self.chars.peek() {
                        if ch == '/' {
                            tokens.push(Token::Comment);
                            self.chars.next();
                            while let Some(&ch) = self.chars.peek() {
                                if ch == '\n' {
                                    break;
                                } else {
                                    self.chars.next();
                                }
                            }
                        } else {
                            tokens.push(Token::Slash);
                        }
                    }
                }
                _ => {
                    let mut word = String::new();
                    while let Some(&ch) = self.chars.peek() {
                        match ch {

                              ' ' | '\n' | '\t' | '\r' | '"' | '='
                            | ';' | ','  | '&'  | '|'  | '!' | ':'
                            | '>' | '{'  | '}'  | '['  | ']' | '/' => { 
                                break;
                            }

                            _ => {
                                word.push(ch);
                                self.chars.next();
                            }
                        }
                    }
                    match word.as_str() {
                        "let" => {
                            tokens.push(Token::Let);
                        }
                        "print" => {
                            tokens.push(Token::Print);
                        }
                        "if" => {
                            tokens.push(Token::If);
                        }
                        "elif" => {
                            tokens.push(Token::Elif);
                        }
                        "else" => {
                            tokens.push(Token::Else);
                        }
                        "for" => {
                            tokens.push(Token::For);
                        }
                        "while" => {
                            tokens.push(Token::While);
                        }
                        "break" => {
                            tokens.push(Token::Break);
                        }                        
                        "-eq" => {
                            tokens.push(Token::EqualTo);
                        }
                        "-le" => {
                            tokens.push(Token::LessEqual);
                        }
                        "-lt" => {
                            tokens.push(Token::LessThan);
                        }
                        "-ge" => {
                            tokens.push(Token::GreaterEqual);
                        }
                        "-gt" => {
                            tokens.push(Token::GreaterThan);
                        }
                        _ => {
                            tokens.push(Token::Word(word));
                        }
                    }
                }
            }
        }
        tokens.push(Token::EOF);
        tokens
    }
}
//
// s = String::from("its just a replacement to f*ck");
// :) = format!("i know it doesn't work", {});
// println!("it worked, fsck /dev/null, ({})", :) );
//
fn main() {
    let file = String::from("cat ~/Downloads/abc/dc.jpg | okay --l > jj --help>> hhff{k} | echo \"my name is {name}\" ");
    let file2 = String::from("let n1 = 43
                              print \"{n1}\"
                              //
                              // i know ky n1-eq43 kaam nhi kry ga kyu ky wo aik hi token bny ga 
                              // ye drawback bash mai bhi hy 
                              //
                              if n1 -eq 43 {
                                  while 1 {
                                      print \"the numbers are eq to: {n1}\"
                                      break
                                  }
                              } elif n1 -le 23 {
                                  print \"wow\" // yhan bhi comment add kr sakte hain
                              } else {
                                  for i in 0; 10 {
                                      print \"the numbers are not eq\"
                                  }
                              }
                              theme 4
                              runitctl --help
                              runitctl enable sshd
                              theme 3 && waybar; hyprpaper& ");
    let mut lexer = Lexer::new(&file);
    let mut tokens = lexer.tokenize();
    println!("{:?}\n", tokens);
    lexer = Lexer::new(&file2);
    tokens = lexer.tokenize();
    println!("{:?}", tokens);
}
//
// :) = String::from("bye");
// EOF :)
// 
