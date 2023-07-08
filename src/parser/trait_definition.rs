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
    .ignore_then(
      sequence()
        .then_ignore(just("_"))
        .then(sequence())
        .map_with_span(|(before, after), span| (span, before + "_" + &after))
    )
    .or_not();

  let member = just("default")
    .padded()
    .or_not()
    .map(|d| d.is_some())
    .then(
      ident()
        .map_with_span(|id, span| (span, id))
        .separated_by(just("|").padded())
        .at_least(1)
    )
    .then(notation)
    .map(|((default, labels), notation)| TraitMember { default, labels, notation });

  let body = member
    .map_with_span(|m, span| (span, m))
    .separated_by(just(",").padded())
    .allow_trailing()
    .at_least(1)
    .then_ignore(whitespace())
    .delimited_by(just("{").padded(), just("}"));

  start
    .ignore_then(ident().map_with_span(|id, span| (span, id)))
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
          label: (13..18, "Place".into()),
          members: vec![
            (29..35, TraitMember { labels: vec![(29..35, "labial".into())], notation: None, default: false }),
            (45..53, TraitMember { labels: vec![(45..53, "alveolar".into())], notation: None, default: false }),
            (63..68, TraitMember { labels: vec![(63..68, "velar".into())], notation: None, default: false }),
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
          label: (13..19, "Stress".into()),
          members: vec![
            (30..42, TraitMember { labels: vec![(30..37, "primary".into())], notation: Some((40..42, "ˈ_".into())), default: false }),
            (52..66, TraitMember { labels: vec![(52..61, "secondary".into())], notation: Some((64..66, "ˌ_".into())), default: false }),
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
          label: (13..19, "Length".into()),
          members: vec![
            (30..43, TraitMember { labels: vec![(38..43, "short".into())], notation: None, default: true }),
            (53..62, TraitMember { labels: vec![(53..57, "long".into())], notation: Some((60..62, "_:".into())), default: false }),
            (72..86, TraitMember { labels: vec![(72..80, "overlong".into())], notation: Some((83..86, "_::".into())), default: false }),
          ],
        },
      )
    )
  }
}
