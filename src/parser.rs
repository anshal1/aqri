pub mod parser {

    use crate::{
        lexer::lexer::{LexToken, SYMBOL},
        statement_parser::{let_parser, print_parser::print_parser},
    };
    #[derive(Debug)]
    pub struct AST {
        pub variable_name: Option<String>,
        pub variable_type: Option<String>,
        pub variable_value: Option<String>,
        pub fn_name: Option<String>,
        pub fn_value: Option<String>,
    }

    pub struct Parser {
        tokens: Vec<LexToken>,
        pos: usize,
    }

    pub fn expect(token: &LexToken, expected_symbol: SYMBOL) -> bool {
        token.symbol == expected_symbol
    }

    impl Parser {
        pub fn new(tokens: Vec<LexToken>) -> Self {
            Parser { tokens, pos: 0 }
        }
        pub fn parse(&mut self) -> Vec<AST> {
            let mut ast_tokens: Vec<AST> = Vec::new();
            loop {
                let token = self.seek();
                if token.symbol == SYMBOL::A_EOF {
                    break;
                }
                match token.symbol {
                    SYMBOL::A_LET => {
                        // consume the starting token before checking the let statement
                        self.seek_and_consume();
                        match let_parser::let_parser::parse_let(self) {
                            Ok(token) => ast_tokens.push(token),
                            Err(e) => {
                                println!("{}", e);
                                break;
                            }
                        }
                    }
                    SYMBOL::A_PRINT_FN => {
                        self.seek_and_consume();
                        match print_parser::parse_print_fn(self) {
                            Ok(token) => ast_tokens.push(token),
                            Err(e) => {
                                println!("{}", e);
                                break;
                            }
                        }
                    }
                    _ => {
                        println!("Unexpected token found: {}", token.val);
                        break;
                    }
                }
            }
            ast_tokens
        }

        pub fn seek(&mut self) -> &LexToken {
            let token = if let Some(tok) = self.tokens.get(self.pos) {
                tok
            } else {
                panic!(
                    "Something went wrong while getting the token at index: {}",
                    self.pos
                );
            };
            token
        }
        pub fn seek_and_consume(&mut self) -> &LexToken {
            let token = if let Some(tok) = self.tokens.get(self.pos) {
                self.pos += 1;
                tok
            } else {
                panic!(
                    "Something went wrong while getting the token at index: {}",
                    self.pos
                );
            };
            token
        }
    }
}
