use chumsky::{prelude::*, text::whitespace};
use crate::{parser::common::*, ast::{Spanned, Series}};
use crate::ast::Stmt;

fn label() -> impl Parser<char, Spanned<String>, Error = Simple<char>> {
  class()
    .map_with_span(|c, span| (span, c.to_string()))
}

pub fn parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
  let start = just("series")
    .padded();

  let list = 
    word_chars()
      .map_with_span(|phoneme, span| (span, phoneme))
      .separated_by(just(",").padded())
      .allow_trailing()
      .at_least(1)
      .then_ignore(whitespace())
      .delimited_by(just("{").padded(), just("}"))
      .map(|phonemes| Series::List(phonemes));
  
  let category = category()
    .map(|cat| Series::Category(cat));

  let body =
    choice([
      list.boxed(),
      category.boxed(),
    ]).map_with_span(|series, span| (span, series));

  start
    .ignore_then(label())
    .then_ignore(just("=").padded())
    .then(body)
    .map(|(label, series)| Stmt::Series { label, series })
}

#[cfg(test)]
mod test {
  use crate::ast::{Category, Feature};

  use super::*;

  #[test]
  fn it_parses_a_list_class_definition() {
    assert_eq!(
      parser().parse("series C = { a, b, c }"),
      Ok(Stmt::Series {
        label: (7..8, "C".into()),
        series: (11..22, Series::List(vec![
          (13..14, "a".into()),
          (16..17, "b".into()),
          (19..20, "c".into()),
        ])),
      })
    )
  }

  #[test]
  fn it_parses_a_category_class_definition() {
    assert_eq!(
      parser().parse("series F = [C+fricative]"),
      Ok(Stmt::Series {
        label: (7..8, "F".into()),
        series: (11..24, Series::Category(Category {
          base_class: Some((12..13, 'C')),
          features: vec![(13..23, Feature::Positive("fricative".into()))]
        })),
      })
    )
  }
}
