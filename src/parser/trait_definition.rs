use chumsky::{prelude::*, text::{ident, whitespace}};
use crate::parser::common::*;
use crate::ast::{
  Stmt,
  TraitMember,
};

pub fn parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
  let start = just("trait")
    .padded();

  let sequence = || word_chars()
    .or_not()
    .map(|opt| match opt {
      Some(ch) => ch,
      None => "".to_string(),
    });

  let notation = just("=")
    .padded()
    .ignore_then(sequence())
    .then_ignore(just("_"))
    .then(sequence())
    .map(|(before, after)| before + "_" + &after)
    .or_not();

  let member = just("default")
    .padded()
    .or_not()
    .map(|d| d.is_some())
    .then(
      ident()
        .separated_by(just("|").padded())
        .at_least(1)
    )
    .then(notation)
    .map(|((default, labels), notation)| TraitMember { default, labels, notation });

  let body = member
    .separated_by(just(",").padded())
    .allow_trailing()
    .at_least(1)
    .then_ignore(whitespace())
    .delimited_by(just("{").padded(), just("}"));

  start
    .ignore_then(ident())
    .then(body)
    .map(|(label, members)| Stmt::Trait { label, members })
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_parses_a_trait() {
    let src = "
      trait Place {
        labial,
        alveolar,
        velar,
      }
    ";
    assert_eq!(
      parser().parse(src.to_string()),
      Ok(
        Stmt::Trait {
          label: "Place".into(),
          members: vec![
            TraitMember { labels: vec!["labial".into()], notation: None, default: false },
            TraitMember { labels: vec!["alveolar".into()], notation: None, default: false },
            TraitMember { labels: vec!["velar".into()], notation: None, default: false },
          ],
        },
      )
    )
  }

  #[test]
  fn it_parses_a_trait_with_annotations() {
    let src = "
      trait Stress {
        primary = ˈ_,
        secondary = ˌ_,
      }
    ";
    assert_eq!(
      parser().parse(src.to_string()),
      Ok(
        Stmt::Trait {
          label: "Stress".into(),
          members: vec![
            TraitMember { labels: vec!["primary".into()], notation: Some("ˈ_".into()), default: false },
            TraitMember { labels: vec!["secondary".into()], notation: Some("ˌ_".into()), default: false },
          ],
        },
      )
    )
  }

  #[test]
  fn it_parses_a_trait_with_a_default() {
    let src = "
      trait Length {
        default short,
        long = _:,
        overlong = _::,
      }
    ";
    assert_eq!(
      parser().parse(src.to_string()),
      Ok(
        Stmt::Trait {
          label: "Length".into(),
          members: vec![
            TraitMember { labels: vec!["short".into()], notation: None, default: true },
            TraitMember { labels: vec!["long".into()], notation: Some("_:".into()), default: false },
            TraitMember { labels: vec!["overlong".into()], notation: Some("_::".into()), default: false },
          ],
        },
      )
    )
  }
}
