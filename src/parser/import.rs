use chumsky::{prelude::*, text::ident};

use crate::ast::Stmt;

pub fn parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
  just("import")
    .padded()
    .ignore_then(
      ident()
        .or(just("*").map(|c| c.to_string()))
        .map_with_span(|name, span| (span, name))
        .separated_by(just("::").ignored())
        .at_least(1)
    )
    .map(|path| Stmt::Import(path))
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_parses_an_import_statement() {
    let src = "import my::phonology";
    assert_eq!(
      parser().parse(src.to_string()),
      Ok(Stmt::Import(vec![
        (7..9, "my".to_string()),
        (11..20, "phonology".to_string())
      ]))
    )
  }

  #[test]
  fn it_parses_an_import_statement_with_star() {
    let src = "import core::ipa::*";
    assert_eq!(
      parser().parse(src.to_string()),
      Ok(Stmt::Import(vec![
        (7..11, "core".to_string()),
        (13..16, "ipa".to_string()),
        (18..19, "*".to_string())
      ]))
    )
  }
}
