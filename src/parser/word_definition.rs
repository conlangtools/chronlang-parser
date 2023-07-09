use crate::parser::common::*;
use chumsky::{
    prelude::*,
    text::{ident, newline, whitespace},
};

use crate::ast::{Definition, Stmt};

fn definition() -> impl Parser<char, Definition, Error = Simple<char>> {
    ident()
        .map_with_span(|pos, span| (span, pos))
        .then_ignore(just("."))
        .padded()
        .or_not()
        .then(description().map_with_span(|def, span| (span, def)))
        .map(|(pos, definition)| Definition { pos, definition })
}

fn definition_block() -> impl Parser<char, Vec<Definition>, Error = Simple<char>> {
    definition()
        .separated_by(newline().then(whitespace()))
        .allow_leading()
        .allow_trailing()
        .at_least(1)
        .delimited_by(just("{"), just("}"))
}

pub fn parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
    let start = just("-").padded();

    let gloss = ident().map_with_span(|g, span| (span, g));

    let pronunciation = syllable()
        .separated_by(just("."))
        .at_least(1)
        .delimited_by(just("/"), just("/"))
        .map_with_span(|p, span| (span, p))
        .padded();

    let definitions =
        whitespace().ignore_then(definition_block().or(definition().map(|d| vec![d])));

    start
        .ignore_then(gloss)
        .then(pronunciation)
        .then(definitions)
        .map(|((gloss, pronunciation), definitions)| Stmt::Word {
            gloss,
            pronunciation,
            definitions,
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_a_word_with_an_inline_definition() {
        let src = "- water /'wa.ter/ noun. the liquid state of H20";
        assert_eq!(
            parser().parse(src.to_string()),
            Ok(Stmt::Word {
                gloss: (2..7, "water".to_string()),
                pronunciation: (8..17, vec!["'wa".to_string(), "ter".to_string()]),
                definitions: vec![Definition {
                    pos: Some((18..22, "noun".to_string())),
                    definition: (24..47, "the liquid state of H20".to_string()),
                }],
            },)
        )
    }

    #[test]
    fn it_parses_a_word_with_one_definition() {
        let src = "
      - water /'wa.ter/ {
        noun. the liquid state of H20
      }
    ";
        assert_eq!(
            parser().parse(src.to_string()),
            Ok(Stmt::Word {
                gloss: (9..14, "water".to_string()),
                pronunciation: (15..24, vec!["'wa".to_string(), "ter".to_string()]),
                definitions: vec![Definition {
                    pos: Some((35..39, "noun".to_string())),
                    definition: (41..64, "the liquid state of H20".to_string()),
                }],
            },)
        )
    }

    #[test]
    fn it_parses_a_word_with_multiple_definitions() {
        let src = "
      - water /'wa.ter/ {
        noun. the liquid state of H20
        verb. to pour water over a plant or area of land
      }
    ";
        assert_eq!(
            parser().parse(src.to_string()),
            Ok(Stmt::Word {
                gloss: (9..14, "water".to_string()),
                pronunciation: (15..24, vec!["'wa".to_string(), "ter".to_string()]),
                definitions: vec![
                    Definition {
                        pos: Some((35..39, "noun".to_string())),
                        definition: (41..64, "the liquid state of H20".to_string()),
                    },
                    Definition {
                        pos: Some((73..77, "verb".to_string())),
                        definition: (
                            79..121,
                            "to pour water over a plant or area of land".to_string()
                        ),
                    },
                ],
            },)
        )
    }
}
