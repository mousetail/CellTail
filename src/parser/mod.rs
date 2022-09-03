use crate::errors;
use crate::lexer::{LexerToken, TokenGroup};
use crate::runtime::attributes;
use crate::runtime::expression::Expression;
use crate::runtime::pattern::Pattern;
use crate::runtime::pattern_list::PatternList;
use crate::tokenizer::{Token, TokenKind};
use std::collections::HashMap;

mod parse_attribute;
mod parse_expression;
mod parse_pattern;

#[derive(Debug)]
pub struct Program {
    pub functions: HashMap<String, PatternList>,
    pub rules: PatternList,
    pub attributes: attributes::Attributes,
}

impl Program {
    fn new() -> Program {
        Program {
            functions: HashMap::new(),
            rules: PatternList(Vec::new()),
            attributes: attributes::Attributes::new(),
        }
    }

    fn add_function_rule(&mut self, function_name: String, rule: (Pattern, Expression)) {
        if let Some(pattern_list) = self.functions.get_mut(&function_name) {
            pattern_list.0.push(rule);
        } else {
            self.functions
                .insert(function_name, PatternList(vec![rule]));
        }
    }

    fn add_rule(&mut self, rule: (Pattern, Expression)) {
        self.rules.0.push(rule);
    }
}

pub fn parse(input: TokenGroup) -> errors::CellTailResult<Program> {
    let mut out = Program::new();
    for statement in input.contents {
        if let LexerToken::Group(group) = statement {
            if group.contains(TokenKind::Equals) {
                parse_attribute::parse_attribute(group, &mut out.attributes)?;
            } else if let Some((pattern, _operator, expression)) =
                group.split_first(TokenKind::Colon)
            {
                if let LexerToken::BasicToken(
                    t @ Token {
                        kind: TokenKind::Identifier,
                        ..
                    },
                ) = &pattern.contents[0]
                {
                    if t.value == "fn" {
                        let function_name = if let LexerToken::BasicToken(
                            t @ Token {
                                kind: TokenKind::Identifier,
                                ..
                            },
                        ) = &pattern.contents[1]
                        {
                            t.value.clone()
                        } else {
                            return Err(errors::CellTailError::new(
                                &pattern,
                                "Expected a function name".to_owned(),
                            ));
                        };
                        out.add_function_rule(
                            function_name,
                            (
                                parse_pattern::parse_as_pattern(TokenGroup {
                                    delimiter: None,
                                    contents: pattern.contents[2..].to_vec(),
                                })?,
                                parse_expression::parse_as_expression(expression)?,
                            ),
                        );

                        continue;
                    }
                }

                out.add_rule((
                    parse_pattern::parse_as_pattern(pattern)?,
                    parse_expression::parse_as_expression(expression)?,
                ))
            } else {
                return Err(errors::CellTailError::new(
                    &group,
                    "Missing : seperating pattern from expression".to_owned(),
                ));
            }
        } else {
            return Err(errors::CellTailError::new(
                &statement,
                "Invalid top-level statement".to_owned(),
            ));
        }
    }

    return Ok(out);
}
