#[derive(Debug)]
pub enum TokenKind {
    Identifier,
    String,
    Number,
    Operator(char),
    OpeningBracket(char),
    ClosingBracket(char),
    Comma,
    Colon,
    Comment,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    start: usize,
    value: String,
    end: usize,
}

fn match_rest<T>(input: &Vec<char>, counter: &mut usize, match_fn: T, kind: TokenKind) -> Token
where
    T: Fn(char) -> bool,
{
    println!(
        "Parsing {kind:?} starting at {counter:?} first char: {:?}",
        input[*counter]
    );
    let start = *counter;
    while *counter < input.len() && match_fn(input[*counter]) {
        *counter += 1;
    }

    //*counter += 1;

    Token {
        kind,
        start: start,
        end: *counter + 1,
        value: input[start..*counter].iter().collect(),
    }
}

pub fn tokenize(input: Vec<char>) -> Vec<Token> {
    let mut counter = 0;
    let mut result: Vec<Token> = vec![];

    while counter < input.len() {
        match input[counter] {
            ' ' | '\n' | '\t' | '\r' => counter += 1,
            '#' => result.push(match_rest(
                &input,
                &mut counter,
                |v: char| v != '\n',
                TokenKind::Comment,
            )),
            '0'..='9' => result.push(match_rest(
                &input,
                &mut counter,
                |v: char| v.is_numeric(),
                TokenKind::Number,
            )),
            '"' => {
                counter += 1;
                result.push(match_rest(
                    &input,
                    &mut counter,
                    |v: char| v != '"',
                    TokenKind::String,
                ));
                counter += 1;
            }
            'a'..='z' | 'A'..='Z' | '_' => result.push(match_rest(
                &input,
                &mut counter,
                |c: char| c.is_alphanumeric() || c == '_',
                TokenKind::Identifier,
            )),
            '(' | '[' | '{' => {
                result.push(Token {
                    kind: TokenKind::OpeningBracket(input[counter]),
                    start: counter,
                    end: counter + 1,
                    value: [input[counter]].iter().collect(),
                });
                counter += 1
            }
            ')' | ']' | '}' => {
                result.push(Token {
                    kind: TokenKind::ClosingBracket(input[counter]),
                    start: counter,
                    end: counter + 1,
                    value: [input[counter]].iter().collect(),
                });
                counter += 1
            }
            x @ ('+' | '-' | '/' | '*' | '&' | '|' | '^' | '%') => result.push(match_rest(
                &input,
                &mut counter,
                |c: char| c == x || c == '@',
                TokenKind::Operator(x),
            )),
            ':' => result.push(Token {
                kind: TokenKind::Colon,
                start: counter,
                end: counter + 1,
                value: [input[counter]].iter().collect(),
            }),
            ',' => result.push(Token {
                kind: TokenKind::Comma,
                start: counter,
                end: counter + 1,
                value: [input[counter]].iter().collect(),
            }),
            _ => {
                todo!()
            }
        }
    }

    result
}
