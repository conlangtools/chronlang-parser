use chumsky::{prelude::*, text::{ident, whitespace}};
use crate::parser::common::*;
use crate::ast::{
  Stmt,
  PhonemeDef,
  Class,
};

pub fn parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
  let start = just("class")
    .padded();

  let encodes = just("encodes")
    .padded()
    .ignore_then(
      ident()
        .separated_by(whitespace())
        .allow_leading()
        .allow_trailing()
        .delimited_by(just("("), just(")"))
    );
  
  let annotates = just("annotates")
    .padded()
    .ignore_then(
      ident()
        .separated_by(whitespace())
        .allow_leading()
        .allow_trailing()
        .delimited_by(just("("), just(")"))
    )
    .or_not()
    .map(|a| match a {
      Some(v) => v,
      None => vec![],
    });

  let phoneme_definition = word_chars()
    .then_ignore(just("=").padded())
    .then(
      ident()
        .separated_by(inline_whitespace())
        .allow_leading()
        .allow_trailing()
    )
    .map(|(label, traits)| PhonemeDef { label, traits });
  
  let body = phoneme_definition
    .separated_by(just(",").padded())
    .allow_trailing()
    .at_least(1)
    .then_ignore(whitespace())
    .delimited_by(just("{").padded(), just("}"));

  let full = start
    .ignore_then(class().map(|c| c.to_string()))
    .then(encodes)
    .then(annotates)
    .then(body)
    .map(|(((label, encodes), annotates), phonemes)| (label, Class::Full { encodes, annotates, phonemes }));

  let list = start
    .ignore_then(class().map(|c| c.to_string()))
    .then_ignore(just("=").padded())
    .then(
      word_chars()
        .separated_by(just(",").padded())
        .allow_trailing()
        .at_least(1)
        .then_ignore(whitespace())
        .delimited_by(just("{").padded(), just("}"))
    )
    .map(|(label, ps)| (label, Class::List(ps)));
  
  let category = start
  .ignore_then(class().map(|c| c.to_string()))
  .then_ignore(just("=").padded())
  .then(category())
  .map(|(label, c)| (label, Class::Category(c)));
  
  choice([
    full.boxed(),
    list.boxed(),
    category.boxed(),
  ])
    .map(|(label, class)| Stmt::Class { label, class })

}

#[cfg(test)]
mod test {
  use crate::ast::{Category, Feature};

use super::*;

  #[test]
  fn it_parses_a_list_class_definition() {
    assert_eq!(
      parser().parse("class C = { a, b, c }"),
      Ok(Stmt::Class {
        label: "C".into(),
        class: Class::List(vec!["a".into(), "b".into(), "c".into()])
      })
    )
  }

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
        label: "C".into(),
        class: Class::Full {
          encodes: vec!["place".into(), "manner".into()],
          annotates: vec![],
          phonemes: vec![
            PhonemeDef { label: "p".into(), traits: vec!["bilabial".into(), "plosive".into()] },
            PhonemeDef { label: "t".into(), traits: vec!["alveolar".into(), "plosive".into()] },
            PhonemeDef { label: "k".into(), traits: vec!["velar".into(), "plosive".into()] },
            PhonemeDef { label: "t͡s".into(), traits: vec!["alveolar".into(), "affricate".into()] },
          ]
        }
      })
    )
  }

  #[test]
  fn it_parses_a_category_class_definition() {
    assert_eq!(
      parser().parse("class F = [C+fricative]"),
      Ok(Stmt::Class {
        label: "F".into(),
        class: Class::Category(Category { base_class: Some('C'), features: vec![Feature::Positive("fricative".into())] })
      })
    )
  }
}
