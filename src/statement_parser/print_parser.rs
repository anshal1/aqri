pub mod print_parser {
    use crate::{
        lexer::lexer::SYMBOL,
        parser::parser::{AST, Parser, expect},
        utils::format_err,
    };

    pub fn parse_print_fn(parser: &mut Parser) -> Result<AST, String> {
        let open_param = parser.seek_and_consume();
        if !expect(open_param, SYMBOL::A_OPEN_PARAN) {
            return Err(format_err("(", &open_param.val));
        };
        let func_val = parser.seek_and_consume();
        let mut fn_val: Option<String> = None;
        if func_val.symbol == SYMBOL::A_IDEN {
            fn_val = Some(String::from(&func_val.val));
            let closing_param = parser.seek_and_consume();
            if !expect(closing_param, SYMBOL::A_CLOSE_PARAN) {
                return Err(format_err(")", &closing_param.val));
            };
            let end = parser.seek_and_consume();
            if !expect(end, SYMBOL::A_STAT_END) {
                return Err(format_err("statement ending character", &end.val));
            }
        } else {
            if !expect(func_val, SYMBOL::A_CLOSE_PARAN) {
                return Err(format_err(")", &func_val.val));
            };
            let end = parser.seek_and_consume();
            if !expect(end, SYMBOL::A_STAT_END) {
                return Err(format_err("statement ending character", &end.val));
            }
        };
        Ok(AST {
            variable_name: None,
            variable_type: None,
            variable_value: None,
            fn_value: fn_val,
            fn_name: Some("prt".to_string()),
        })
    }
}
