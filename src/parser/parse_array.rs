use crate::errors;
use crate::lexer::TokenGroup;
use crate::tokenizer::TokenKind;

pub fn parse_array<F, G, B>(
    input: TokenGroup,
    parser: F,
    mut combiner: G,
    array_tail: B,
) -> errors::CellTailResult<B>
where
    F: FnMut(TokenGroup) -> errors::CellTailResult<B>,
    G: FnMut(B, B) -> errors::CellTailResult<B>,
{
    if input.delimiter != Some('[') {
        return Err(errors::CellTailError::new(
            &input,
            format!("Expected a list to be delimited by []"),
        ));
    }

    input
        .split_all(TokenKind::Comma)
        .into_iter()
        .rev()
        .map(parser)
        .try_fold(array_tail, |a, b| combiner(b?, a))
}
