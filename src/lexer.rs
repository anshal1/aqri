pub mod lexer {
    use phf::phf_map;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum SYMBOL {
        A_LET,
        A_IDEN,
        A_TYP,
        A_EQ,
        A_VAL,
        A_OPEN_PARAN,
        A_CLOSE_PARAN,
        A_STAT_END,
        A_PRINT_FN,
        A_EOF,
    }
    #[derive(Clone, Debug)]
    pub struct LexToken {
        pub symbol: SYMBOL,
        pub val: String,
    }

    static SYMBOL_MAP: phf::Map<&str, SYMBOL> = phf_map! {
    "let" => SYMBOL::A_LET,
    "int" => SYMBOL::A_TYP,
    "=" => SYMBOL::A_EQ,
    "(" => SYMBOL::A_OPEN_PARAN,
    ")" => SYMBOL::A_CLOSE_PARAN,
    "prt" => SYMBOL::A_PRINT_FN,
    "end" => SYMBOL::A_STAT_END,
    "str" => SYMBOL::A_TYP
    };

    pub struct Lexer {
        input: String,
        pos: usize,
    }

    pub fn is_letter(char: &str) -> bool {
        (char >= "a" && char <= "z") || (char >= "A" && char <= "Z") || (char == "_")
    }
    fn is_digit(char: &str) -> bool {
        char >= "0" && char <= "9"
    }

    impl Lexer {
        pub fn new(input: String) -> Self {
            Lexer { input, pos: 0 }
        }

        pub fn create_tokens(&mut self) -> Vec<LexToken> {
            let mut token: Vec<LexToken> = Vec::new();
            loop {
                let curr_char = self.seek();
                // println!("CURRENT: {}", curr_char);
                if curr_char == " " {
                    self.seek_and_consume();
                    continue;
                };
                if curr_char == "end" {
                    token.push(LexToken {
                        symbol: SYMBOL::A_STAT_END,
                        val: "".to_string(),
                    });
                    return token;
                }
                match curr_char.as_str() {
                    "=" => {
                        token.push(LexToken {
                            symbol: SYMBOL::A_EQ,
                            val: curr_char,
                        });
                        self.seek_and_consume();
                    }
                    "(" => {
                        token.push(LexToken {
                            symbol: SYMBOL::A_OPEN_PARAN,
                            val: curr_char,
                        });
                        self.seek_and_consume();
                    }
                    ")" => {
                        token.push(LexToken {
                            symbol: SYMBOL::A_CLOSE_PARAN,
                            val: curr_char,
                        });
                        self.seek_and_consume();
                    }
                    "\"" => {
                        token.push(self.read_str());
                    }
                    s if is_letter(s) => {
                        token.push(self.read_idn());
                    }
                    s if is_digit(s) => {
                        token.push(self.read_num());
                    }
                    _ => {
                        // even if the token is unkown read it, throwing an error fot this is the part of the parser
                        token.push(self.read_idn());
                    }
                }
            }
        }

        fn read_idn(&mut self) -> LexToken {
            let mut idn = String::new();
            while self.seek() != " " && is_letter(&self.seek()) {
                let curr_char = self.seek_and_consume();
                idn.push_str(&curr_char);
            }
            let symbol = SYMBOL_MAP.get(&idn).cloned();
            if let Some(sym) = symbol {
                return {
                    LexToken {
                        symbol: sym,
                        val: idn,
                    }
                };
            } else {
                return LexToken {
                    symbol: SYMBOL::A_IDEN,
                    val: idn,
                };
            };
        }
        fn read_num(&mut self) -> LexToken {
            let mut val = String::new();
            while is_digit(&self.seek()) {
                let curr_char = self.seek_and_consume();
                val.push_str(&curr_char);
            }
            // skipping over the ending double quotes
            self.seek_and_consume();
            return LexToken {
                symbol: SYMBOL::A_VAL,
                val,
            };
        }

        fn read_str(&mut self) -> LexToken {
            let mut string = String::new();
            // push the starting quote into the string as consume it so the loop below can run without failing
            string.push_str(self.seek_and_consume().as_str());
            while self.seek() != "\"" {
                string.push_str(&self.seek_and_consume());
            }
            string.push_str("\"");
            // consume the ending quote
            self.seek_and_consume();
            LexToken {
                symbol: SYMBOL::A_VAL,
                val: string,
            }
        }

        pub fn seek_and_consume(&mut self) -> String {
            if self.pos >= self.input.len() {
                return "end".to_string();
            };
            let char = &self.input[self.pos..self.pos + 1];
            self.pos += char.len();
            String::from(char)
        }
        pub fn seek(&mut self) -> String {
            if self.pos >= self.input.len() {
                return "end".to_string();
            };
            let char = &self.input[self.pos..self.pos + 1];
            String::from(char)
        }
    }
}
