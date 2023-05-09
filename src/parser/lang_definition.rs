use chumsky::{prelude::*, text::{ident, Character}};
use crate::ast::Stmt;

pub fn parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
  let start = just("lang").padded();

  let id = ident();

  let parent = just("<")
    .padded()
    .ignore_then(ident())
    .or_not();

  let name = just(":")
    .padded()
    .ignore_then(filter(|c: &char| c.is_alphanumeric() || c.is_inline_whitespace() || "-()".contains(*c)).repeated().at_least(1))
    .map(|cs| cs.iter().collect())
    .or_not();

  start
    .ignore_then(id)
    .then(parent)
    .then(name)
    .map(|((id, parent), name)| Stmt::Language { id, parent, name })
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_parses_a_language() {
    let src = "lang PA";
    assert_eq!(
      parser().parse(src.to_string()),
      Ok(Stmt::Language { id: "PA".to_string(), parent: None, name: None })
    )
  }

  #[test]
  fn it_parses_a_language_with_a_parent() {
    let src = "lang OA < PA";
    assert_eq!(
      parser().parse(src.to_string()),
      Ok(Stmt::Language { id: "OA".to_string(), parent: Some("PA".to_string()), name: None })
    )
  }

  #[test]
  fn it_parses_a_language_with_a_name() {
    let src = "lang PA: Proto-A";
    assert_eq!(
      parser().parse(src.to_string()),
      Ok(Stmt::Language { id: "PA".to_string(), parent: None, name: Some("Proto-A".to_string()) })
    )
  }

  #[test]
  fn it_parses_a_language_with_a_parent_and_a_name() {
    let src = "lang OA < PA: Old A";
    assert_eq!(
      parser().parse(src.to_string()),
      Ok(Stmt::Language { id: "OA".to_string(), parent: Some("PA".to_string()), name: Some("Old A".to_string()) })
    )
  }
}
