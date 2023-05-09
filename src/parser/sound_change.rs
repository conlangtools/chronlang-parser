use chumsky::prelude::*;

use crate::parser::common::*;

use crate::ast::{
  Segment,
  Pattern,
  Spanned,
  Source,
  Target,
  EnvPattern,
  EnvElement,
  Environment,
  Stmt,
};

fn segment() -> impl Parser<char, Segment, Error = Simple<char>> {
  choice([
    word_chars().map(|cs| Segment::Phonemes(cs)).boxed(),
    category().map(|c| Segment::Category(c)).boxed(),
  ])
}

fn pattern() -> impl Parser<char, Pattern, Error = Simple<char>> {
  segment().repeated().at_least(1)
}

fn empty_source() -> impl Parser<char, Source, Error = Simple<char>> {
  just("[]").map(|_| Source::Empty)
}

fn source() -> impl Parser<char, Spanned<Source>, Error = Simple<char>> {
  choice([
    pattern().map(Source::Pattern).boxed(),
    empty_source().boxed(),
  ])
  .map_with_span(|source, span| (span, source))
}

fn modification() -> impl Parser<char, Target, Error = Simple<char>> {
  feature()
    .repeated()
    .at_least(1)
    .delimited_by(just("["), just("]"))
    .map(Target::Modification)
}

fn empty_target() -> impl Parser<char, Target, Error = Simple<char>> {
  just("[]").map(|_| Target::Empty)
}

fn target() -> impl Parser<char, Spanned<Target>, Error = Simple<char>> {
  choice([
    modification().boxed(),
    pattern().map(Target::Pattern).boxed(),
    empty_target().boxed(),
  ])
  .map_with_span(|target, span| (span, target))
}

fn env_pattern() -> impl Parser<char, EnvPattern, Error = Simple<char>> {
  segment().map(EnvElement::Segment)
    .or(just(".").map(|_| EnvElement::SyllableBoundary))
    .or(just("#").map(|_| EnvElement::WordBoundary))
    .repeated().at_least(1)
}

fn environment() -> impl Parser<char, Spanned<Environment>, Error = Simple<char>> {
  env_pattern().or_not()
    .then_ignore(just("_"))
    .then(env_pattern().or_not())
    .map_with_span(|(before, after), span| (span, Environment { before, after }))
}

pub fn parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
  let start = just("$").padded();

  let source = source();

  let target =
    just(">")
    .padded()
    .ignore_then(target());

  let environment =
    just("/")
    .padded()
    .ignore_then(environment())
    .or_not();

  let description =
    just(":")
    .padded()
    .ignore_then(
      description()
        .map_with_span(|desc, span| (span, desc))
    )
    .or_not();

  start
    .ignore_then(source)
    .then(target)
    .then(environment)
    .then(description)
    .map(|(((source, target), environment), description)| Stmt::SoundChange { source, target, environment, description })
}

#[cfg(test)]
mod test {
  use super::*;

  use crate::ast::{
    Category,
    Feature,
  };

  #[test]
  fn it_parses_a_sound_change() {
    let src = "$ ɢ > g";
    let res = parser().parse(src.to_string());
    assert_eq!(
      res,
      Ok(
        Stmt::SoundChange {
          source: (2..3, Source::Pattern(vec![Segment::Phonemes("ɢ".into())])),
          target: (6..7, Target::Pattern(vec![Segment::Phonemes("g".into())])),
          environment: None,
          description: None,
        }
      )
    )
  }

  #[test]
  fn it_parses_a_sound_change_with_an_environment() {
    let src = "$ k > c / _[V+close]";
    assert_eq!(
      parser().parse(src.to_string()),
      Ok(
        Stmt::SoundChange {
          source: (2..3, Source::Pattern(vec![Segment::Phonemes("k".into())])),
          target: (6..7, Target::Pattern(vec![Segment::Phonemes("c".into())])),
          environment: Some((10..20, Environment {
            before: None,
            after: Some(vec![
              EnvElement::Segment(Segment::Category(Category {
                base_class: Some('V'),
                features: vec![Feature::Positive("close".to_string())]
              }))
            ])
          })),
          description: None,
        }
      )
    )
  }

  #[test]
  fn it_parses_a_sound_change_with_an_environment_and_description() {
    let src = "$ k > c / #_i : Word-initial k lenites to c before i";
    assert_eq!(
      parser().parse(src.to_string()),
      Ok(
        Stmt::SoundChange {
          source: (2..3, Source::Pattern(vec![Segment::Phonemes("k".into())])),
          target: (6..7, Target::Pattern(vec![Segment::Phonemes("c".into())])),
          environment: Some((10..13, Environment {
            before: Some(vec![EnvElement::WordBoundary]),
            after: Some(vec![EnvElement::Segment(Segment::Phonemes("i".into()))])
          })),
          description: Some((16..52, "Word-initial k lenites to c before i".to_string())),
        }
      )
    )
  }

  #[test]
  fn it_parses_a_sound_change_with_categories_and_modifications() {
    let src = "$ [C+stop+alveolar] > [+flap]";
    assert_eq!(
      parser().parse(src.to_string()),
      Ok(
        Stmt::SoundChange {
          source: (
            2..19,
            Source::Pattern(vec![Segment::Category(Category {
              base_class: Some('C'),
              features: vec![Feature::Positive("stop".to_string()), Feature::Positive("alveolar".to_string())]
            })])
          ),
          target: (
            22..29,
            Target::Modification(vec![Feature::Positive("flap".to_string())])
          ),
          environment: None,
          description: None,
        }
      )
    )
  }

  #[test]
  fn it_parses_a_sound_change_with_empty_source_or_target() {
    let src = "$ [] > []";
    assert_eq!(
      parser().parse(src.to_string()),
      Ok(
        Stmt::SoundChange {
          source: (2..4, Source::Empty),
          target: (7..9, Target::Empty),
          environment: None,
          description: None,
        }
      )
    )
  }

  #[test]
  fn it_does_not_parse_a_sound_change_with_no_source() {
    let src = "$  > [] / _";
    let res= parser().parse(src.to_string());
    
    assert!(res.is_err());
  }

  #[test]
  fn it_does_not_parse_a_sound_change_with_no_target() {
    let src = "$ [] > / _";
    let res= parser().parse(src.to_string());
    
    assert!(res.is_err());
  }
}
