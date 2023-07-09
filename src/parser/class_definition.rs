use crate::ast::{PhonemeDef, Stmt};
use crate::parser::common::*;
use chumsky::{
    prelude::*,
    text::{ident, whitespace},
};

pub fn parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
    let label = class().map_with_span(|c, span| (span, c.to_string()));

    let start = just("class").padded();

    let encodes = just("encodes").padded().ignore_then(
        ident()
            .map_with_span(|id, span| (span, id))
            .separated_by(whitespace())
            .allow_leading()
            .allow_trailing()
            .delimited_by(just("("), just(")")),
    );

    let annotates = just("annotates")
        .padded()
        .ignore_then(
            ident()
                .map_with_span(|id, span| (span, id))
                .separated_by(whitespace())
                .allow_leading()
                .allow_trailing()
                .delimited_by(just("("), just(")")),
        )
        .or_not()
        .map(|a| match a {
            Some(v) => v,
            None => vec![],
        });

    let phoneme_definition = word_chars()
        .map_with_span(|phoneme, span| (span, phoneme))
        .then_ignore(just("=").padded())
        .then(
            ident()
                .map_with_span(|traits, span| (span, traits))
                .separated_by(inline_whitespace())
                .allow_leading()
                .allow_trailing(),
        )
        .map(|(label, traits)| PhonemeDef { label, traits });

    let body = phoneme_definition
        .map_with_span(|ph, span| (span, ph))
        .separated_by(just(",").padded())
        .allow_trailing()
        .at_least(1)
        .then_ignore(whitespace())
        .delimited_by(just("{").padded(), just("}"));

    start
        .ignore_then(label)
        .then(encodes)
        .then(annotates)
        .then(body)
        .map(|(((label, encodes), annotates), phonemes)| Stmt::Class {
            label,
            encodes,
            annotates,
            phonemes,
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_a_full_class_definition() {
        assert_eq!(
            parser().parse(
                "class C encodes (place manner) {
          p = bilabial plosive,
          t = alveolar plosive,
          k = velar plosive,
          t͡s = alveolar affricate,
        }"
            ),
            Ok(Stmt::Class {
                label: (6..7, "C".into()),
                encodes: vec![(17..22, "place".into()), (23..29, "manner".into())],
                annotates: vec![],
                phonemes: vec![
                    (
                        43..63,
                        PhonemeDef {
                            label: (43..44, "p".into()),
                            traits: vec![(47..55, "bilabial".into()), (56..63, "plosive".into())]
                        }
                    ),
                    (
                        75..95,
                        PhonemeDef {
                            label: (75..76, "t".into()),
                            traits: vec![(79..87, "alveolar".into()), (88..95, "plosive".into())]
                        }
                    ),
                    (
                        107..124,
                        PhonemeDef {
                            label: (107..108, "k".into()),
                            traits: vec![(111..116, "velar".into()), (117..124, "plosive".into())]
                        }
                    ),
                    (
                        136..160,
                        PhonemeDef {
                            label: (136..139, "t͡s".into()),
                            traits: vec![
                                (142..150, "alveolar".into()),
                                (151..160, "affricate".into())
                            ]
                        }
                    ),
                ]
            })
        )
    }
}
