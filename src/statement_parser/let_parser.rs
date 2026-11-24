pub mod let_parser {
    use crate::{
        lexer::lexer::SYMBOL,
        parser::parser::{AST, Parser, expect},
    };

    pub fn parse_let(parser: &mut Parser) -> Result<AST, String> {
        let name = parser.seek_and_consume();
        if !expect(name, SYMBOL::A_IDEN) {
            return Err(format!("Expected variable name, instead got: {}", name.val));
        }
        let name = name.clone();
        let type_token = parser.seek_and_consume();
        if !expect(type_token, SYMBOL::A_TYP) {
            return Err(format!(
                "Expected variable type eg int, str. instead got: {}",
                type_token.val
            ));
        }
        let type_token = type_token.clone();
        let token = parser.seek_and_consume();
        if !expect(token, SYMBOL::A_EQ) {
            return Err(format!("Expected a =, instead got: {}", token.val));
        }

        let value = parser.seek_and_consume();
        if !expect(value, SYMBOL::A_VAL) {
            return Err(format!("Expected a value, instead got: {}", value.val));
        }
        let value = value.clone();
        let end = parser.seek_and_consume();
        if !expect(end, SYMBOL::A_STAT_END) {
            return Err(format!(
                "Expected statement end character which is a new line character, instead got: {}",
                end.val
            ));
        }
        let ast = AST {
            variable_name: Some(name.val),
            variable_type: Some(type_token.val),
            variable_value: Some(value.val),
            fn_name: None,
            fn_value: None,
        };
        Ok(ast)
    }
}
