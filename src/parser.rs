use chumsky::{prelude::*, text::{newline, whitespace}};

use crate::ast::{Stmt, Spanned};

mod class_definition;
mod series_definition;
mod common;
mod import;
mod lang_definition;
mod sound_change;
mod trait_definition;
mod word_definition;
mod milestone;

fn stmt() -> impl Parser<char, Spanned<Stmt>, Error = Simple<char>> {
  choice([
    sound_change::parser().boxed(),
    import::parser().boxed(),
    lang_definition::parser().boxed(),
    word_definition::parser().boxed(),
    trait_definition::parser().boxed(),
    class_definition::parser().boxed(),
    series_definition::parser().boxed(),
    milestone::parser().boxed(),
  ])
    .map_with_span(|stmt, span| (span, stmt))
    .then_ignore(newline().repeated().at_least(1).ignored().or(end()))
    .then_ignore(whitespace())
}

fn root() -> impl Parser<char, Vec<Spanned<Stmt>>, Error = Simple<char>> {
  stmt()
    .repeated()
    .padded()
    .then_ignore(end())
}

pub fn parse(source: &str) -> Result<Vec<(std::ops::Range<usize>, Stmt)>, Vec<chumsky::error::Simple<char>>> {
  root().parse(source)
}
