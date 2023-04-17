use chumsky::{prelude::*, text::{newline, ident, whitespace}};
use crate::parser::common::*;

use crate::ast::{
  Definition,
  Stmt,
};

fn definition() -> impl Parser<char, Definition, Error = Simple<char>> {
  ident()
    .then_ignore(just("."))
    .padded()
    .or_not()
    .then(description())
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

  let gloss = ident();

  let pronunciation = syllable()
    .separated_by(just("."))
    .at_least(1)
    .delimited_by(just("/"), just("/"))
    .padded();

  let definitions = whitespace().ignore_then(
    definition_block()
      .or(definition().map(|d| vec![d]))
  );

  start
    .ignore_then(gloss)
    .then(pronunciation)
    .then(definitions)
    .map(|((gloss, pronunciation), definitions)| Stmt::Word { gloss, pronunciation, definitions })
}

#[cfg(test)]
mod test{
  use super::*;

  #[test]
  fn it_parses_a_word_with_an_inline_definition() {
    let src = "- water /'wa.ter/ noun. the liquid state of H20";
    assert_eq!(
      parser().parse(src.to_string()),
      Ok(
        Stmt::Word {
          gloss: "water".to_string(),
          pronunciation: vec!["'wa".to_string(), "ter".to_string()],
          definitions: vec![Definition { pos: Some("noun".to_string()), definition: "the liquid state of H20".to_string() }],
        },
      )
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
      Ok(
        Stmt::Word {
          gloss: "water".to_string(),
          pronunciation: vec!["'wa".to_string(), "ter".to_string()],
          definitions: vec![Definition { pos: Some("noun".to_string()), definition: "the liquid state of H20".to_string() }],
        },
      )
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
      Ok(
        Stmt::Word {
          gloss: "water".to_string(),
          pronunciation: vec!["'wa".to_string(), "ter".to_string()],
          definitions: vec![
            Definition { pos: Some("noun".to_string()), definition: "the liquid state of H20".to_string() },
            Definition { pos: Some("verb".to_string()), definition: "to pour water over a plant or area of land".to_string() },
          ],
        },
      )
    )
  }
}
