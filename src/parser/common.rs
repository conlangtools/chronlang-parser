use crate::ast::{Category, Feature};
use chumsky::{
    prelude::*,
    text::{ident, Character},
};

pub fn inline_whitespace() -> impl Parser<char, (), Error = Simple<char>> {
    filter(|c: &char| c.is_inline_whitespace())
        .repeated()
        .ignored()
}

pub fn word_char() -> impl Parser<char, char, Error = Simple<char>> {
    filter(|c: &char| !"/.,[]{}()>+-_#".contains(*c) && !c.is_whitespace())
}

pub fn word_chars() -> impl Parser<char, String, Error = Simple<char>> {
    word_char()
        .repeated()
        .at_least(1)
        .map(|cs| cs.iter().collect())
}

pub fn class() -> impl Parser<char, char, Error = Simple<char>> {
    filter(|c: &char| c.is_ascii_uppercase())
}

pub fn description() -> impl Parser<char, String, Error = Simple<char>> {
    filter(|c: &char| !"\r\n{}".contains(*c))
        .repeated()
        .at_least(1)
        .map(|cs| cs.iter().collect())
}

pub fn syllable() -> impl Parser<char, String, Error = Simple<char>> {
    word_char()
        .repeated()
        .at_least(1)
        .map(|cs| cs.iter().collect())
}

pub fn feature() -> impl Parser<char, Feature, Error = Simple<char>> {
    let sign = filter(|c: &char| "+-".contains(*c));
    let ident = ident();

    sign.then(ident).padded().map(|(s, i)| match s {
        '+' => Feature::Positive(i),
        _ => Feature::Negative(i),
    })
}

pub fn category() -> impl Parser<char, Category, Error = Simple<char>> {
    class()
        .map_with_span(|class, span| (span, class))
        .padded()
        .or_not()
        .then(
            feature()
                .map_with_span(|feat, span| (span, feat))
                .repeated()
                .at_least(1),
        )
        .delimited_by(just("["), just("]"))
        .map(|(base_class, features)| Category {
            base_class,
            features,
        })
}

pub fn integer() -> impl Parser<char, i64, Error = Simple<char>> {
    filter(|c: &char| c.is_numeric())
        .repeated()
        .at_least(1)
        .try_map(|cs, span| {
            cs.iter()
                .collect::<String>()
                .parse::<i64>()
                .map_err(|e| Simple::custom(span, format!("{}", e)))
        })
}
